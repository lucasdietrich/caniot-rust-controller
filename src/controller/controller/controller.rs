use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;


use itertools::{partition, Itertools};
use tokio::runtime::Runtime;
use tokio::sync::oneshot::{Sender};

use crate::bus::{CanInterface, CanInterfaceError};
use crate::caniot::emu::{
    emu_pool2_realistic_add_devices_to_iface,
};
use crate::caniot::{self, Frame, RequestData};
use crate::caniot::{DeviceId, Request};
use crate::controller::actor::ControllerMessage;
use crate::controller::{
    ActionVerdict, Device, DeviceAction, DeviceActionResult, DeviceError, DeviceTrait,
    DeviceWrapperTrait, PendingAction, ProcessContext, Verdict,
};
use crate::shutdown::Shutdown;

use super::pending_action;
use super::pending_query::PendingQueryTenant;

use super::super::{actor};
use super::attach::device_attach_controller;
use super::PendingQuery;

use log::{info, warn};
use serde::{Deserialize, Serialize};


use thiserror::Error;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

const QUERY_DEFAULT_TIMEOUT_MS: u32 = 1000; // 1s
const ACTION_DEFAULT_TIMEOUT_MS: u32 = QUERY_DEFAULT_TIMEOUT_MS; // 1s

const CHANNEL_SIZE: usize = 10;
const DEVICES_COUNT: usize = 63;

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct ControllerStats {
    pub rx: usize,
    pub tx: usize,
    pub err: usize,
    pub malformed: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaniotConfig {}

impl Default for CaniotConfig {
    fn default() -> Self {
        CaniotConfig {}
    }
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
}

enum ActionResultOrPending {
    // Action Result
    Result(DeviceActionResult),

    // Pending Action on Device (DID)
    Pending(DeviceAction, Frame<RequestData>),
}

pub struct Controller {
    // CAN interface
    pub iface: CanInterface,

    // Service
    pub config: CaniotConfig,
    pub stats: ControllerStats,
    rt: Arc<Runtime>,
    shutdown: Shutdown,
    receiver: mpsc::Receiver<actor::ControllerMessage>,
    handle: actor::ControllerHandle,

    // State
    pending_queries: Vec<PendingQuery>,
    devices: HashMap<DeviceId, Device>,
}

impl Controller {
    pub(crate) fn new(
        mut iface: CanInterface,
        config: CaniotConfig,
        shutdown: Shutdown,
        rt: Arc<Runtime>,
    ) -> Result<Self, ControllerError> {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);

        #[cfg(feature = "emu")]
        {
            // emu_pool1_add_devices_to_iface(&mut iface);
            emu_pool2_realistic_add_devices_to_iface(&mut iface);
        }

        Ok(Self {
            iface,
            config,
            stats: ControllerStats::default(),
            rt,
            shutdown,
            receiver,
            handle: actor::ControllerHandle::new(sender),
            pending_queries: Vec::new(),
            devices: HashMap::new(),
        })
    }

    pub fn get_handle(&self) -> actor::ControllerHandle {
        self.handle.clone()
    }

    pub async fn iface_send_caniot_frame(
        iface: &mut CanInterface,
        stats: &mut ControllerStats,
        request: &caniot::Request,
    ) -> Result<(), ControllerError> {
        info!("TX {}", request);

        let can_frame = request.into();

        iface.send(can_frame).await?;
        stats.tx += 1;
        Ok(())
    }

    pub async fn send_caniot_frame(
        &mut self,
        request: &caniot::Request,
    ) -> Result<(), ControllerError> {
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
        }
    }

    async fn handle_caniot_frame(
        &mut self,
        frame: caniot::Response,
    ) -> Result<(), ControllerError> {
        self.stats.rx += 1;
        let mut answered_pending_action: Option<PendingAction> = None;

        // Find pending queries that can be answered by this frame
        // TODO broadcast should be handled differently as the oneshot channel cannot be used to send multiple responses
        let pivot = self
            .pending_queries
            .partition_point(|pq| pq.match_response(&frame));

        // if a frame can answer multiple pending queries, remove all of them
        for pq in self.pending_queries.drain(..pivot) {
            if let Some(pq_tenant) = pq.end_with_frame(frame.clone()) {
                match pq_tenant {
                    PendingQueryTenant::Action(pending_action) => {
                        answered_pending_action.replace(pending_action);
                    }
                    _ => {}
                }
            }
        }

        // Get or create device
        let device = if let Some(device) = self.devices.get_mut(&frame.device_id) {
            device
        } else {
            let mut new_device = Device::new(frame.device_id);
            device_attach_controller(&mut new_device);
            self.devices.insert(frame.device_id, new_device);
            self.devices.get_mut(&frame.device_id).unwrap()
        };
        let device_did = device.did;

        let mut device_context = ProcessContext::default();

        // Let the device handle the frame
        let verdict = device.handle_frame(&frame.data, &mut device_context)?;
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
        if let Some(answered_action) = answered_pending_action {
            let action_result = device.handle_action_result(&answered_action.action)?;
            answered_action.send(Ok(action_result));
        }

        Ok(())
    }

    fn time_to_next_pq_timeout(&self) -> Duration {
        // TODO improve this code
        let now = std::time::Instant::now();

        let neareast_timeout = self
            .pending_queries
            .iter()
            .map(|pq| pq.get_timeout_instant())
            .min();

        if let Some(neareast_timeout) = neareast_timeout {
            if neareast_timeout > now {
                neareast_timeout - now
            } else {
                Duration::from_millis(0)
            }
        } else {
            Duration::MAX
        }
    }

    fn time_to_next_device_process(&self) -> Duration {
        let now = std::time::Instant::now();

        let nearest_process = self
            .devices
            .iter()
            .filter_map(|(_did, device)| {
                if let Some(next_process) = device.next_process_time() {
                    Some(next_process)
                } else {
                    None
                }
            })
            .min();

        if let Some(nearest_process) = nearest_process {
            if nearest_process > now {
                nearest_process - now
            } else {
                Duration::from_millis(0)
            }
        } else {
            Duration::MAX
        }
    }

    async fn handle_pending_queries_timeout(&mut self) {
        let now = std::time::Instant::now();

        // place all timed out queries at the end of the vector
        let split_index = partition(&mut self.pending_queries, |pq| !pq.has_timed_out(&now));

        // remove timed out queries
        let timed_out_queries = self.pending_queries.split_off(split_index);

        // send timeout to all timed out queries
        for pq in timed_out_queries {
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
            let time_to_next_pq_timeout = self.time_to_next_pq_timeout();
            let time_to_next_device_process = self.time_to_next_device_process();
            let sleep_time = time_to_next_pq_timeout.min(time_to_next_device_process);

            select! {
                Some(message) = self.receiver.recv() => {
                    let _ = self.handle_api_message(message).await;
                },
                Some(frame) = self.iface.recv_poll() => {
                    match caniot::Response::try_from(frame) {
                        Ok(frame) => {
                            info!("RX {}", frame);
                            let result = self.handle_caniot_frame(frame).await;
                            if let Err(err) = result {
                                error!("Failed to handle CANIOT frame {}", err);
                            }
                        },
                        Err(err) => {
                            self.stats.malformed += 1;
                            error!("Failed to convert into CANIOT frame {}", err)
                        },
                    }
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
            self.process_devices().await.unwrap();
        }

        Ok(())
    }

    fn get_devices_stats(&self) -> Vec<actor::DeviceStatsEntry> {
        self.devices
            .iter()
            .filter(|(_did, device)| device.last_seen.is_some())
            .sorted_by_key(|(_did, device)| device.last_seen.unwrap())
            .rev()
            .map(|(_did, device)| actor::DeviceStatsEntry {
                did: device.did,
                last_seen: device.last_seen,
                stats: device.stats,
            })
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
                    .filter(|device| {
                        if let Some(ref device_inner) = device.inner {
                            device_inner.wrapper_can_handle_action(&**inner_action)
                        } else {
                            false
                        }
                    })
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
            Some(did) => {
                let device = self.get_device_by_did(&did)?;
                if !device.wrapper_can_handle_action(&action) {
                    return Err(DeviceError::UnsupportedAction.into());
                }
                Ok(device)
            }
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
        match message {
            ControllerMessage::GetStats { respond_to } => {
                let _ =
                    respond_to.send((self.stats, self.get_devices_stats(), self.iface.get_stats()));
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
        }

        Ok(())
    }
}
