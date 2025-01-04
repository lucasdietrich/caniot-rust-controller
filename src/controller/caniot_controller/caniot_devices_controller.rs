use std::collections::hash_map::Entry;
use std::collections::HashMap;

use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use futures::future::Pending;
use itertools::{partition, Itertools};

use socketcan::CanDataFrame;
use tokio::sync::oneshot::Sender;

use crate::bus::{CanInterfaceError, CanInterfaceTrait, CAN_IOCTL_SEND_EMU_EVENT};
use crate::caniot::{self, are_requests_concurrent, Frame, RequestData};
use crate::caniot::{DeviceId, Request};
use crate::controller::caniot_controller::api_message::CaniotApiMessage;
use crate::controller::caniot_controller::auto_attach::device_init_controller;
use crate::controller::caniot_controller::pending_action::PendingAction;
use crate::controller::caniot_controller::pending_query::{PendingQuery, PendingQueryTenant};
use crate::controller::{
    ActionVerdict, CaniotConfig, CaniotDevicesConfig, Device, DeviceAction, DeviceActionResult,
    DeviceError, DeviceInfos, ProcessContext, Verdict,
};
use crate::database::{SettingsStore, Storage};
use crate::utils::expirable::{ttl, ExpirableTrait};

#[cfg(feature = "can-tunnel")]
use super::can_tunnel::CanTunnelContextServer;
use super::device_filter::DeviceFilter;
use super::stats::CaniotControllerStats;

use log::{info, warn};

use thiserror::Error;

const PENDING_QUERY_DEFAULT_TIMEOUT_MS: u32 = 1000; // 1s
const ACTION_DEFAULT_TIMEOUT_MS: u32 = PENDING_QUERY_DEFAULT_TIMEOUT_MS; // 1s

#[derive(Error, Debug)]
pub enum CaniotControllerError {
    #[error("Timeout Error")]
    Timeout,

    #[error("Unsupported query Error")]
    UnsupportedQuery,

    #[error("Duplicate device Error")]
    #[allow(dead_code)]
    DuplicateDID,

    #[error("Duplicate pending query: response undifferentiable")]
    UndifferentiablePendingQuery,

    #[error("Generic device action needs a device ID")]
    GenericDeviceActionNeedsDID,

    #[error("Not implemented")]
    #[allow(dead_code)]
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

pub struct CaniotDevicesController<IF: CanInterfaceTrait> {
    // Database storage
    storage: Arc<Storage>,

    // Service
    pub config: CaniotConfig,
    pub stats: CaniotControllerStats,

    // can interface
    pub iface: IF,

    // caniot devices
    pending_queries: Vec<PendingQuery>,
    devices: HashMap<DeviceId, Device>, // caniot devices

    #[cfg(feature = "can-tunnel")]
    tunnel_server: CanTunnelContextServer,
}

impl<IF: CanInterfaceTrait> CaniotDevicesController<IF> {
    pub(crate) fn new(
        iface: IF,
        config: CaniotConfig,
        storage: Arc<Storage>,
    ) -> Result<Self, CaniotControllerError> {
        Ok(Self {
            iface,
            storage,
            config,
            stats: CaniotControllerStats::default(),

            pending_queries: Vec::new(),
            devices: HashMap::new(),
            #[cfg(feature = "can-tunnel")]
            tunnel_server: CanTunnelContextServer::default(),
        })
    }

    pub async fn start(&mut self) -> Result<(), CaniotControllerError> {
        self.request_telemetry_broadcast().await
    }

    pub async fn iface_send_caniot_frame(
        iface: &mut impl CanInterfaceTrait,
        stats: &mut CaniotControllerStats,
        request: &caniot::Request,
    ) -> Result<(), CaniotControllerError> {
        info!("TX {}", request);

        let can_frame = request.into();

        iface.send(can_frame).await?;
        stats.iface_tx += 1;
        Ok(())
    }

    async fn device_get_or_create<'d, 'stg>(
        devices: &'d mut HashMap<DeviceId, Device>,
        did: DeviceId,
        devices_config: &CaniotDevicesConfig,
        stg: SettingsStore<'stg>,
    ) -> &'d mut Device {
        match devices.entry(did) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                // Create controller for device
                let device_controller = device_init_controller(did, (), devices_config, stg).await;

                // Create device and attach controller if any
                let new_device = Device::new(did, device_controller);

                // Insert device in the devices map
                entry.insert(new_device)
            }
        }
    }

    pub async fn send_caniot_frame(
        &mut self,
        request: &caniot::Request,
    ) -> Result<(), CaniotControllerError> {
        if request.is_broadcast() {
            self.stats.broadcast_tx += 1;
        } else {
            // Get or instantiate device
            let device = Self::device_get_or_create(
                &mut self.devices,
                request.device_id,
                &self.config.devices,
                self.storage.get_settings_store(),
            )
            .await;

            // update device stats
            match request.data {
                RequestData::Telemetry { .. } => device.stats.telemetry_tx += 1,
                RequestData::Command { .. } => device.stats.command_tx += 1,
                RequestData::AttributeRead { .. } => device.stats.attribute_rx += 1,
                RequestData::AttributeWrite { .. } => device.stats.attribute_tx += 1,
            }
        }

        Self::iface_send_caniot_frame(&mut self.iface, &mut self.stats, request).await
    }

    async fn send_pend_request(
        &mut self,
        request: caniot::Request,
        timeout_ms: Option<u32>,
        tenant: PendingQueryTenant,
    ) {
        let timeout_ms = timeout_ms.unwrap_or(
            self.config
                .pending_queries_default_timeout
                .unwrap_or(PENDING_QUERY_DEFAULT_TIMEOUT_MS),
        );

        if request.device_id == DeviceId::BROADCAST {
            error!("BROADCAST query not supported");
            let _ = tenant.end_with_error(CaniotControllerError::UnsupportedQuery);
        } else if self
            .pending_queries
            .iter()
            .any(|pq| are_requests_concurrent(&pq.query, &request))
        {
            // Attempt to send the same query multiple times:
            // queries for which response cannot be differentiated must not
            // be sent as pending queries multiple times.
            error!("Duplicate pending query: response undifferentiable, request not sent");
            self.stats.pq_duplicate_dropped += 1;
            tenant.end_with_error(CaniotControllerError::UndifferentiablePendingQuery);
        } else if let Err(err) = self.send_caniot_frame(&request).await {
            error!("Failed to send CANIOT frame: {:?}", err);
            let _ = tenant.end_with_error(err);
        } else {
            self.pending_queries
                .push(PendingQuery::new(request, timeout_ms, tenant));
            self.stats.pq_pushed += 1;
        }
    }

    async fn device_update_from_context<'f>(
        device: &mut Device,
        ctx: ProcessContext<'f>,
    ) -> Result<(), DeviceError> {
        if ctx.request_jobs_update {
            device.update_scheduled_jobs();
        }
        device.register_new_jobs(ctx.new_jobs);
        if let Some(device_future) = ctx.storage_update_future {
            device_future.await.map_err(|err| {
                error!("Failed to run device future: {}", err);
                err
            })?;
        }

        Ok(())
    }

    pub async fn handle_caniot_frame(
        &mut self,
        frame: caniot::Response,
    ) -> Result<(), CaniotControllerError> {
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
                            panic!(
                                "
                                Multiple concurrent requests pending for the same device,
                                current implement does not support multiple pending
                                requests with undifferentiable responses
                                for a given device.
                                Please refer to ControllerError::UndifferentiablePendingQuery
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
        let device = Self::device_get_or_create(
            &mut self.devices,
            device_did,
            &self.config.devices,
            self.storage.get_settings_store(),
        )
        .await;
        let mut device_ctx = ProcessContext::new(Some(frame.timestamp), self.storage.clone());

        // Let the device handle the frame
        let verdict = device.handle_frame(&frame.data, &None, &mut device_ctx)?;
        match verdict {
            Verdict::None => {}
            Verdict::Request(request) => {
                let request = Request::new(device_did, request);
                Self::iface_send_caniot_frame(&mut self.iface, &mut self.stats, &request).await?;
            }
        }

        Self::device_update_from_context(device, device_ctx).await?;

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

    async fn handle_pending_queries_timeout(&mut self, now: &std::time::Instant) {
        // place all timed out queries at the end of the vector
        let split_index = partition(&mut self.pending_queries, |pq| !pq.has_timed_out(now));

        // remove timed out queries
        let timed_out_queries = self.pending_queries.split_off(split_index);

        // send timeout to all timed out queries
        for pq in timed_out_queries {
            self.stats.pq_timeout += 1;
            warn!(
                "Pending query {} timed out after {} ms",
                pq.query, pq.timeout_ms
            );
            pq.end_with_error(CaniotControllerError::Timeout);
        }
    }

    // Process all devices expired jobs
    async fn process_devices_jobs(
        &mut self,
        now: &DateTime<Utc>,
    ) -> Result<(), CaniotControllerError> {
        let storage = self.storage.clone();
        for (did, device) in self
            .devices
            .iter_mut()
            .filter(|(_, device)| device.is_expired(now))
        {
            // Calculate triggered jobs
            device.shift_jobs(now);

            // Process device jobs until no more jobs are available
            loop {
                let mut device_ctx = ProcessContext::new(None, storage.clone());

                if let Some(verdict) = device.process_one_job(&mut device_ctx).transpose()? {
                    match verdict {
                        Verdict::None => {}
                        Verdict::Request(request) => {
                            let request = Request::new(*did, request);
                            Self::iface_send_caniot_frame(
                                &mut self.iface,
                                &mut self.stats,
                                &request,
                            )
                            .await?;
                        }
                    }

                    Self::device_update_from_context(device, device_ctx).await?;
                } else {
                    break;
                }
            }
        }

        Ok(())
    }

    // Return a list of devices with given filter
    fn get_devices_infos(&self, filter: DeviceFilter) -> Vec<DeviceInfos> {
        let filter_function = filter.get_filter_function();
        let sort_function = filter.get_sort_function();
        self.devices
            .iter()
            .filter(|(_, device)| filter_function(device))
            .sorted_by(|(_, a), (_, b)| sort_function(a, b))
            .map(|(_, device)| device.into())
            .rev()
            .collect()
    }

    fn get_device_by_did(&mut self, did: &DeviceId) -> Result<&mut Device, CaniotControllerError> {
        self.devices
            .get_mut(did)
            .ok_or(CaniotControllerError::NoSuchDevice)
    }

    fn get_device_by_action(
        &mut self,
        action: &DeviceAction,
    ) -> Result<&mut Device, CaniotControllerError> {
        match action {
            DeviceAction::Inner(inner_action) => {
                let mut devices_candidates: Vec<&mut Device> = self
                    .devices
                    .values_mut()
                    .filter(|device| device.can_inner_controller_handle_action(&**inner_action))
                    .collect();

                match devices_candidates.len() {
                    0 => Err(CaniotControllerError::NoSuchDeviceForAction),
                    1 => Ok(devices_candidates.swap_remove(0)),
                    _ => Err(CaniotControllerError::MultipleDevicesForAction),
                }
            }
            _ => Err(CaniotControllerError::GenericDeviceActionNeedsDID),
        }
    }

    async fn handle_api_device_action(
        &mut self,
        did: Option<DeviceId>,
        action: DeviceAction,
        respond_to: Sender<Result<DeviceActionResult, CaniotControllerError>>,
        timeout_ms: Option<u32>,
    ) {
        let result = self.handle_api_device_action_inner(did, action).await;

        match result {
            Ok(ActionResultOrPending::Result(result)) => {
                let _ = respond_to.send(Ok(result));
            }
            Ok(ActionResultOrPending::Pending(action, request)) => {
                let tenant = PendingQueryTenant::Action(PendingAction::new(action, respond_to));
                self.send_pend_request(
                    request,
                    Some(
                        timeout_ms.unwrap_or(
                            self.config
                                .action_default_timeout
                                .unwrap_or(ACTION_DEFAULT_TIMEOUT_MS),
                        ),
                    ),
                    tenant,
                )
                .await;
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
    ) -> Result<ActionResultOrPending, CaniotControllerError> {
        let storage = self.storage.clone();
        // Find device by DID or by action
        let device = match did {
            Some(did) => Ok(self.get_device_by_did(&did)?),
            None => self.get_device_by_action(&action),
        }?;

        let mut device_ctx = ProcessContext::new(None, storage);
        let result = match device.handle_action(&action, &mut device_ctx) {
            Ok(verdict) => match verdict {
                ActionVerdict::ActionPendingOn(request) => {
                    let request = Request::new(device.did, request);
                    Ok(ActionResultOrPending::Pending(action, request))
                }
                ActionVerdict::ActionResult(result) => Ok(ActionResultOrPending::Result(result)),
                ActionVerdict::ActionRejected(reason) => Err(DeviceError::ActionRejected(reason)),
            },
            Err(err) => Err(err),
        }
        .map_err(CaniotControllerError::from);

        Self::device_update_from_context(device, device_ctx).await?;

        result
    }

    pub async fn handle_api_message(
        &mut self,
        message: CaniotApiMessage,
    ) -> Result<(), CaniotControllerError> {
        match message {
            CaniotApiMessage::GetDevices { filter, respond_to } => {
                let _ = respond_to.send(self.get_devices_infos(filter));
            }
            CaniotApiMessage::Query {
                query,
                timeout_ms,
                respond_to,
            } => {
                if let Some(respond_to) = respond_to {
                    let tenant = PendingQueryTenant::Query(respond_to);
                    self.send_pend_request(query, timeout_ms, tenant).await;
                } else {
                    let _ = self.send_caniot_frame(&query).await;
                }
            }
            CaniotApiMessage::DeviceAction {
                did,
                action,
                respond_to,
                timeout_ms,
            } => {
                self.handle_api_device_action(did, action, respond_to, timeout_ms)
                    .await;
            }
            CaniotApiMessage::DevicesResetMeasuresStats => {
                for device in self.devices.values_mut() {
                    device.reset_controller_measures_stats();
                }
            }
            CaniotApiMessage::DevicesResetSettings { respond_to } => {
                for device in self.devices.values_mut() {
                    let mut ctx = ProcessContext::new(None, self.storage.clone());
                    device.reset_settings(&mut ctx);
                    Self::device_update_from_context(device, ctx).await?;
                }
                let _ = respond_to.send(Ok(()));
            }
            #[cfg(feature = "can-tunnel")]
            CaniotApiMessage::EstablishCanTunnel {
                rx_queue,
                tx_queue,
                respond_to,
            } => {
                let result = self
                    .tunnel_server
                    .establish_can_tunnel(rx_queue, tx_queue)
                    .map_err(Into::into);
                let _ = respond_to.send(result);
            }
            #[cfg(feature = "emu")]
            CaniotApiMessage::EmulationRequest { event } => self
                .iface
                .ioctl(CAN_IOCTL_SEND_EMU_EVENT, Into::<i32>::into(event) as u32)?,
        }

        Ok(())
    }

    async fn request_telemetry_broadcast(&mut self) -> Result<(), CaniotControllerError> {
        let frame = RequestData::Telemetry {
            endpoint: caniot::Endpoint::BoardControl,
        }
        .into_broadcast();
        self.send_caniot_frame(&frame).await
    }

    pub async fn handle_can_frame(&mut self, frame: CanDataFrame) {
        // Send frame to tunnel if established
        #[cfg(feature = "can-tunnel")]
        self.tunnel_server.notify_rx(frame.clone());

        // Process the frame in the current controller
        match caniot::Response::try_from(frame) {
            Ok(frame) => {
                info!("RX {}", frame);

                let result = self.handle_caniot_frame(frame).await;
                if let Err(err) = result {
                    error!("Failed to handle CANIOT frame {}", err);
                }
            }
            Err(err) => {
                self.stats.iface_malformed += 1;
                error!("Failed to convert into CANIOT frame: {}", err)
            }
        }
    }

    pub async fn loop_process(&mut self, sys_now: &Instant, utc_now: &DateTime<Utc>) -> Duration {
        let sleep_time = ttl(&[
            self.pending_queries.iter().ttl(sys_now),
            self.devices.values().ttl(&utc_now).map(|chrono_duration| {
                chrono_duration
                    .to_std()
                    .expect("Failed to convert chrono duration to std duration")
            }),
        ])
        .unwrap_or(Duration::MAX);

        self.handle_pending_queries_timeout(&sys_now).await;

        let result = self.process_devices_jobs(&utc_now).await;
        if let Err(err) = result {
            error!("Failed to process devices: {}", err);
        }

        sleep_time
    }

    pub fn tunnel_poll_tx(&mut self) -> Pending<Option<CanDataFrame>> {
        let tunnel_poll_tx: Pending<Option<CanDataFrame>> = {
            #[cfg(feature = "can-tunnel")]
            {
                self.tunnel_server.poll_tx()
            }
            #[cfg(not(feature = "can-tunnel"))]
            {
                futures::future::pending()
            }
        };

        tunnel_poll_tx
    }
}
