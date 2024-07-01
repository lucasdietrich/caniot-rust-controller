use std::collections::HashMap;

use std::time::Duration;

use itertools::{partition, Itertools};

use socketcan::CanDataFrame;
use tokio::sync::oneshot::Sender;

use crate::bus::{CanInterfaceError, CanInterfaceTrait, CAN_IOCTL_SEND_EMU_EVENT};
use crate::caniot::{self, Frame, RequestData};
use crate::caniot::{DeviceId, Request};
use crate::controller::handle::{ControllerMessage, DeviceFilter};
use crate::controller::{
    ActionVerdict, Device, DeviceAction, DeviceActionResult, DeviceError, DeviceInfos,
    PendingAction, ProcessContext, Verdict,
};
use crate::shutdown::Shutdown;
use crate::utils::expirable::{ttl, ExpirableTrait};

#[cfg(feature = "can-tunnel")]
use super::can_tunnel::CanTunnelContext;
use super::pending_query::PendingQueryTenant;
use super::{pending_action, CaniotConfig};

use super::super::handle;
use super::auto_attach::device_attach_controller;
use super::PendingQuery;

use log::{info, warn};
use serde::Serialize;

use thiserror::Error;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

const QUERY_DEFAULT_TIMEOUT_MS: u32 = 1000; // 1s
const ACTION_DEFAULT_TIMEOUT_MS: u32 = QUERY_DEFAULT_TIMEOUT_MS; // 1s

const CHANNEL_SIZE: usize = 10;
// const DEVICES_COUNT: usize = 63;

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct ControllerStats {
    // can interface
    pub iface_rx: usize,
    pub iface_tx: usize,
    pub iface_err: usize,
    pub iface_malformed: usize,
    // dropped ?

    // Pending queries
    pub pq_pushed: usize,
    pub pq_timeout: usize,
    pub pq_answered: usize,

    // API
    pub api_rx: usize,
}

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Timeout Error")]
    Timeout,

    #[error("Unsupported query Error")]
    UnsupportedQuery,

    #[error("Duplicate device Error")]
    DuplicateDID,

    #[error("Generic device action needs a device ID")]
    GenericDeviceActionNeedsDID,

    #[error("Not implemented")]
    NotImplemented,

    #[error("Unknown device")]
    NoSuchDevice,

    #[error("No such device can handle the action")]
    NoSuchDeviceForAction,

    #[error("Multiple devices can handle the action")]
    MultipleDevicesForAction,

    #[error("CAN Interface Error: {0}")]
    CanError(#[from] CanInterfaceError),

    #[error("CANIOT Error: {0}")]
    CaniotProtocolError(#[from] caniot::ProtocolError),

    #[error("Conversion Error: {0}")]
    CaniotConversionError(#[from] caniot::ConversionError),

    #[error("Device error: {0}")]
    DeviceError(#[from] DeviceError),

    #[cfg(feature = "can-tunnel")]
    #[error("Can tunnel error: {0}")]
    CanTunnelError(#[from] super::can_tunnel::CanTunnelError),
}

enum ActionResultOrPending {
    // Action Result
    Result(DeviceActionResult),

    // Pending Action on Device (DID)
    Pending(DeviceAction, Frame<RequestData>),
}

pub struct Controller<IF: CanInterfaceTrait> {
    // CAN interface
    pub iface: IF,

    // Service
    pub config: CaniotConfig,
    pub stats: ControllerStats,
    shutdown: Shutdown,
    receiver: mpsc::Receiver<handle::ControllerMessage>,
    handle: handle::ControllerHandle,

    // State
    pending_queries: Vec<PendingQuery>,
    devices: HashMap<DeviceId, Device>,

    #[cfg(feature = "can-tunnel")]
    tunnel: CanTunnelContext,
}

impl<IF: CanInterfaceTrait> Controller<IF> {
    pub(crate) fn new(
        iface: IF,
        config: CaniotConfig,
        shutdown: Shutdown,
    ) -> Result<Self, ControllerError> {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);

        Ok(Self {
            iface,
            config,
            stats: ControllerStats::default(),
            shutdown,
            receiver,
            handle: handle::ControllerHandle::new(sender),
            pending_queries: Vec::new(),
            devices: HashMap::new(),
            #[cfg(feature = "can-tunnel")]
            tunnel: CanTunnelContext::default(),
        })
    }

    pub fn get_handle(&self) -> handle::ControllerHandle {
        self.handle.clone()
    }

    pub async fn iface_send_caniot_frame(
        iface: &mut impl CanInterfaceTrait,
        stats: &mut ControllerStats,
        request: &caniot::Request,
    ) -> Result<(), ControllerError> {
        info!("TX {}", request);

        let can_frame = request.into();

        iface.send(can_frame).await?;
        stats.iface_tx += 1;
        Ok(())
    }

    fn device_get_or_create(devices: &mut HashMap<DeviceId, Device>, did: DeviceId) -> &mut Device {
        devices.entry(did).or_insert_with(|| {
            let mut new_device = Device::new(did);
            device_attach_controller(&mut new_device);
            new_device
        })
    }

    pub async fn send_caniot_frame(
        &mut self,
        request: &caniot::Request,
    ) -> Result<(), ControllerError> {
        // Get or instantiate device
        let device_did = request.device_id;
        let device = Self::device_get_or_create(&mut self.devices, device_did);

        // update device stats
        match request.data {
            RequestData::Telemetry { .. } => device.stats.telemetry_tx += 1,
            RequestData::Command { .. } => device.stats.command_tx += 1,
            RequestData::AttributeRead { .. } => device.stats.attribute_rx += 1,
            RequestData::AttributeWrite { .. } => device.stats.attribute_tx += 1,
        }

        Self::iface_send_caniot_frame(&mut self.iface, &mut self.stats, request).await
    }

    pub async fn query_sched(
        &mut self,
        request: caniot::Request,
        timeout_ms: Option<u32>,
        sender: oneshot::Sender<Result<caniot::Response, ControllerError>>,
    ) {
        let timeout_ms = timeout_ms.unwrap_or(QUERY_DEFAULT_TIMEOUT_MS);

        if request.device_id == DeviceId::BROADCAST {
            error!("BROADCAST query not supported");
            let _ = sender.send(Err(ControllerError::UnsupportedQuery));
        } else if let Err(err) = self.send_caniot_frame(&request).await {
            error!("Failed to send CANIOT frame: {:?}", err);
            let _ = sender.send(Err(err)); // Send None, but do not panic if receiver is dropped
        } else {
            self.pending_queries.push(PendingQuery::new(
                request,
                timeout_ms,
                super::pending_query::PendingQueryTenant::Query(sender),
            ));
            self.stats.pq_pushed += 1;
        }
    }

    async fn handle_caniot_frame(
        &mut self,
        frame: caniot::Response,
    ) -> Result<(), ControllerError> {
        self.stats.iface_rx += 1;
        // TODO if multiple actions are pending, only the first one will be answered
        // For the other, the channel sender will be dropped and the response will be lost
        let mut answered_pending_action: Option<PendingAction> = None;

        // Find pending queries that can be answered by this frame
        // TODO broadcast should be handled differently as the oneshot channel cannot be used to send multiple responses
        let pivot = self
            .pending_queries
            .partition_point(|pq| pq.match_response(&frame));

        // if a frame can answer multiple pending queries, remove all of them
        for pq in self.pending_queries.drain(..pivot) {
            self.stats.pq_answered += 1;
            if let Some(pq_tenant) = pq.end_with_frame(frame.clone()) {
                match pq_tenant {
                    PendingQueryTenant::Action(pending_action) => {
                        if answered_pending_action.replace(pending_action).is_some() {
                            // TODO
                            panic!(
                                "
                                Multiple actions pending on the same frame, \
                                only the first one will be answered, 
                                the channel sender of the following pendings will be dropped
                                causing:

                                thread 'tokio-runtime-worker' panicked at src/controller/handle.rs:96:24:
                                IPC Sender dropped before response: RecvError(())

                                This can be easily reproduced by delaying the reponse of an emulated device
                                and sending the same action multiple times.

                                This can be solved by make 'answered_pending_action' a Vec<PendingAction>
                                and store all pending actions that have been answered by the frame
                                "
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        // Get or create device
        let device_did = frame.device_id;
        let device = Self::device_get_or_create(&mut self.devices, device_did);

        let mut device_context = ProcessContext::default();

        // Let the device handle the frame
        let verdict = device.handle_frame(&frame.data, &None, &mut device_context)?;
        match verdict {
            Verdict::None => {}
            Verdict::Request(request) => {
                let request = Request::new(device_did, request);
                Self::iface_send_caniot_frame(&mut self.iface, &mut self.stats, &request).await?;
            }
        }

        // Set next process time
        device.schedule_next_process_in(device_context.next_process);

        // Let the device compute the action result if any
        if let Some(mut answered_action) = answered_pending_action {
            let completed_by = answered_action
                .response
                .take()
                .expect("Response not set for action");
            let action_result =
                device.handle_action_result(&answered_action.action, completed_by)?;
            answered_action.send(Ok(action_result));
        }

        Ok(())
    }

    async fn handle_pending_queries_timeout(&mut self) {
        let now = std::time::Instant::now();

        // place all timed out queries at the end of the vector
        let split_index = partition(&mut self.pending_queries, |pq| !pq.has_timed_out(&now));

        // remove timed out queries
        let timed_out_queries = self.pending_queries.split_off(split_index);

        // send timeout to all timed out queries
        for pq in timed_out_queries {
            self.stats.pq_timeout += 1;
            warn!(
                "Pending query {} timed out after {} ms",
                pq.query, pq.timeout_ms
            );
            pq.end_with_error(ControllerError::Timeout);
        }
    }

    async fn process_devices(&mut self) -> Result<(), ControllerError> {
        // Use iterator directly to avoid unnecessary indexing and cloning
        for (did, device) in self.devices.iter_mut() {
            if device.needs_process() {
                // Check if device needs processing (requested by controller)
                let mut device_context = ProcessContext::default();
                let verdict = device.process(&mut device_context)?;
                device.mark_processed();
                device.schedule_next_process_in(device_context.next_process);

                match verdict {
                    Verdict::None => {}
                    Verdict::Request(request) => {
                        let request = Request::new(*did, request);
                        Self::iface_send_caniot_frame(&mut self.iface, &mut self.stats, &request)
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn run(mut self) -> Result<(), ()> {
        loop {
            let sleep_time = ttl(&[
                self.pending_queries.iter().ttl(),
                self.devices.values().ttl(),
            ])
            .unwrap_or(Duration::MAX);

            let tunnel_poll_tx = {
                #[cfg(feature = "can-tunnel")]
                {
                    self.tunnel.poll_tx()
                }
                #[cfg(not(feature = "can-tunnel"))]
                {
                    futures::future::pending()
                }
            };

            select! {
                Some(message) = self.receiver.recv() => {
                    let _ = self.handle_api_message(message).await;
                },
                Some(frame) = self.iface.recv_poll() => {
                    // Send frame to tunnel if established
                    #[cfg(feature = "can-tunnel")]
                    self.tunnel.notify_rx(frame.clone());

                    // Process the frame in the curren controller
                    match caniot::Response::try_from(frame) {
                        Ok(frame) => {
                            info!("RX {}", frame);
                            let result = self.handle_caniot_frame(frame).await;
                            if let Err(err) = result {
                                error!("Failed to handle CANIOT frame {}", err);
                            }
                        },
                        Err(err) => {
                            self.stats.iface_malformed += 1;
                            error!("Failed to convert into CANIOT frame {}", err)
                        },
                    }
                },
                Some(frame) = tunnel_poll_tx => {
                    // If frame is received from tunnel, send it to the bus
                    let _ = self.iface.send(frame).await;
                },
                _ = sleep(sleep_time) => {
                    // Timeout of pending queries handled in handle_pending_queries_timeout()
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal, exiting ...");
                    break;
                }
            }

            self.handle_pending_queries_timeout().await;

            let result = self.process_devices().await;
            if let Err(err) = result {
                error!("Failed to process devices: {}", err);
            }
        }

        Ok(())
    }

    fn get_devices_infos(&self, filter: DeviceFilter) -> Vec<DeviceInfos> {
        let filter_function = filter.get_filter_function();
        self.devices
            .iter()
            .filter(|(_, device)| filter_function(device))
            .map(|(_, device)| device.into())
            .collect()
    }

    fn get_device_by_did(&mut self, did: &DeviceId) -> Result<&mut Device, ControllerError> {
        self.devices
            .get_mut(did)
            .ok_or(ControllerError::NoSuchDevice)
    }

    fn get_device_by_action(
        &mut self,
        action: &DeviceAction,
    ) -> Result<&mut Device, ControllerError> {
        match action {
            DeviceAction::Inner(inner_action) => {
                let mut devices_candidates: Vec<&mut Device> = self
                    .devices
                    .values_mut()
                    .filter(|device| device.can_inner_controller_handle_action(&**inner_action))
                    .collect();

                match devices_candidates.len() {
                    0 => Err(ControllerError::NoSuchDeviceForAction),
                    1 => Ok(devices_candidates.swap_remove(0)),
                    _ => Err(ControllerError::MultipleDevicesForAction),
                }
            }
            _ => Err(ControllerError::GenericDeviceActionNeedsDID),
        }
    }

    async fn handle_api_device_action(
        &mut self,
        did: Option<DeviceId>,
        action: DeviceAction,
        respond_to: Sender<Result<DeviceActionResult, ControllerError>>,
        timeout_ms: Option<u32>,
    ) {
        let result = self.handle_api_device_action_inner(did, action).await;

        match result {
            Ok(ActionResultOrPending::Result(result)) => {
                let _ = respond_to.send(Ok(result));
            }
            Ok(ActionResultOrPending::Pending(action, request)) => {
                // TODO improve sending pending action
                let _ = self.send_caniot_frame(&request).await;

                self.pending_queries.push(PendingQuery::new(
                    request,
                    timeout_ms.unwrap_or(ACTION_DEFAULT_TIMEOUT_MS),
                    super::pending_query::PendingQueryTenant::Action(
                        pending_action::PendingAction::new(action, respond_to),
                    ),
                ));
                self.stats.pq_pushed += 1;
            }
            Err(err) => {
                let _ = respond_to.send(Err(err));
            }
        }
    }

    async fn handle_api_device_action_inner(
        &mut self,
        did: Option<DeviceId>,
        action: DeviceAction,
    ) -> Result<ActionResultOrPending, ControllerError> {
        // Find device by DID or by action
        let device = match did {
            Some(did) => Ok(self.get_device_by_did(&did)?),
            None => self.get_device_by_action(&action),
        }?;

        let mut device_context = ProcessContext::default();
        match device.handle_action(&action, &mut device_context) {
            Ok(verdict) => {
                device.schedule_next_process_in(device_context.next_process);
                let did = device.did;
                let result_or_pending = match verdict {
                    ActionVerdict::ActionPendingOn(request) => {
                        let request = Request::new(did, request);
                        ActionResultOrPending::Pending(action, request)
                    }
                    ActionVerdict::ActionResult(result) => ActionResultOrPending::Result(result),
                };

                Ok(result_or_pending)
            }
            Err(err) => Err(err),
        }
        .map_err(ControllerError::from)
    }

    pub async fn handle_api_message(
        &mut self,
        message: ControllerMessage,
    ) -> Result<(), ControllerError> {
        self.stats.api_rx += 1;
        match message {
            ControllerMessage::GetControllerStats { respond_to } => {
                let _ = respond_to.send((self.stats, self.iface.get_stats()));
            }
            ControllerMessage::GetDevices { filter, respond_to } => {
                let _ = respond_to.send(self.get_devices_infos(filter));
            }
            ControllerMessage::Query {
                query,
                timeout_ms,
                respond_to,
            } => {
                if let Some(respond_to) = respond_to {
                    self.query_sched(query, timeout_ms, respond_to).await;
                } else {
                    let _ = self.send_caniot_frame(&query).await;
                }
            }
            ControllerMessage::DeviceAction {
                did,
                action,
                respond_to,
                timeout_ms,
            } => {
                self.handle_api_device_action(did, action, respond_to, timeout_ms)
                    .await;
            }
            #[cfg(feature = "can-tunnel")]
            ControllerMessage::EstablishCanTunnel {
                rx_queue,
                tx_queue,
                respond_to,
            } => {
                let result = self
                    .tunnel
                    .establish_can_tunnel(rx_queue, tx_queue)
                    .map_err(Into::into);
                let _ = respond_to.send(result);
            }
            #[cfg(feature = "emu")]
            ControllerMessage::EmulationEvent { event } => self
                .iface
                .ioctl(CAN_IOCTL_SEND_EMU_EVENT, Into::<i32>::into(event) as u32)?,
        }

        Ok(())
    }
}
