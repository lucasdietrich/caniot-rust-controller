use std::{cmp::Ordering, ops::Deref};

use as_any::Downcast;
use chrono::{DateTime, Utc};

use tokio::sync::{mpsc, oneshot};

use crate::{
    bus::CanStats,
    caniot::{self as ct, DeviceId},
    grpcserver::EmuRequest,
};
use serde::Serialize;

use super::{
    alert, ActionTrait, ControllerError, ControllerStats, Device, DeviceAction, DeviceActionResult,
    DeviceInfos, DeviceStats,
};

#[derive(Debug, Default)]
pub enum DeviceFilter {
    #[default]
    All, // All devices sorted by did
    ById(DeviceId),  // A single device
    WithActiveAlert, // Devices with active alerts sorted by alert severity (highest first)
}

impl DeviceFilter {
    pub fn get_filter_function<'a>(&'a self) -> Box<dyn Fn(&Device) -> bool + 'a> {
        match self {
            DeviceFilter::All => Box::new(|_| true),
            DeviceFilter::ById(did) => Box::new(move |device| device.did == *did),
            DeviceFilter::WithActiveAlert => Box::new(|device| device.get_alert().is_some()),
        }
    }

    pub fn get_sort_function<'a>(&'a self) -> Box<dyn Fn(&Device, &Device) -> Ordering + 'a> {
        match self {
            DeviceFilter::All => Box::new(|a, b| a.did.cmp(&b.did)),
            DeviceFilter::ById(_) => Box::new(|_, _| Ordering::Equal),
            DeviceFilter::WithActiveAlert => {
                Box::new(|a, b| alert::cmp_severity(&a.get_alert(), &b.get_alert()))
            }
        }
    }
}

pub enum ControllerMessage {
    GetControllerStats {
        respond_to: oneshot::Sender<(ControllerStats, CanStats)>,
    },
    GetDevices {
        filter: DeviceFilter,
        respond_to: oneshot::Sender<Vec<DeviceInfos>>,
    },
    Query {
        query: ct::Request,
        timeout_ms: Option<u32>,

        // If Some, the controller will respond to the sender with the result of the query.
        // If None, the controller will send the query and not wait for a response.
        respond_to: Option<oneshot::Sender<Result<ct::Response, ControllerError>>>,
    },
    DeviceAction {
        did: Option<DeviceId>,
        action: DeviceAction,
        respond_to: oneshot::Sender<Result<<DeviceAction as ActionTrait>::Result, ControllerError>>,
        timeout_ms: Option<u32>,
    },
    #[cfg(feature = "can-tunnel")]
    EstablishCanTunnel {
        rx_queue: mpsc::Sender<CanDataFrame>, // Messages received from the bus
        tx_queue: mpsc::Receiver<CanDataFrame>, // Messages to sent to the bus
        respond_to: oneshot::Sender<Result<(), ControllerError>>,
    },
    #[cfg(feature = "emu")]
    EmulationRequest { event: EmuRequest },
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    sender: mpsc::Sender<ControllerMessage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceStatsEntry {
    pub did: ct::DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub stats: DeviceStats,
}

impl ControllerHandle {
    pub fn new(sender: mpsc::Sender<ControllerMessage>) -> Self {
        Self { sender }
    }

    pub async fn device_request(
        &self,
        frame: ct::Request,
        timeout_ms: Option<u32>,
    ) -> Result<ct::Response, ControllerError> {
        self.query(|sender| ControllerMessage::Query {
            query: frame,
            timeout_ms,
            respond_to: Some(sender),
        })
        .await
    }

    /// Query a controller message
    ///
    /// Create a one-shot channel, embed it in a message using the provided closure, and send the
    /// message to the controller actor. Wait for the response and return it.
    async fn query<R>(
        &self,
        build_message_closure: impl FnOnce(oneshot::Sender<R>) -> ControllerMessage,
    ) -> R {
        let (sender, receiver) = oneshot::channel();
        let message = build_message_closure(sender);
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
        receiver.await.expect("IPC Sender dropped before response")
    }

    pub async fn get_controller_stats(&self) -> (ControllerStats, CanStats) {
        self.query(|respond_to| ControllerMessage::GetControllerStats { respond_to })
            .await
    }

    pub async fn get_devices_infos_list(&self) -> Vec<DeviceInfos> {
        self.query(|respond_to| ControllerMessage::GetDevices {
            filter: DeviceFilter::All,
            respond_to,
        })
        .await
    }

    pub async fn get_devices_with_active_alert(&self) -> Vec<DeviceInfos> {
        self.query(|respond_to| ControllerMessage::GetDevices {
            filter: DeviceFilter::WithActiveAlert,
            respond_to,
        })
        .await
    }

    pub async fn get_device_infos(&self, did: DeviceId) -> Option<DeviceInfos> {
        self.query(|respond_to| ControllerMessage::GetDevices {
            filter: DeviceFilter::ById(did),
            respond_to,
        })
        .await
        .into_iter()
        .next()
    }

    // Send a generic device action to the controller of the device.
    // Some actions are unique to a specific device, so the device id is optional.
    // For generic actions, the device id is required.
    pub async fn device_action(
        &self,
        did: Option<DeviceId>,
        action: DeviceAction,
        timeout_ms: Option<u32>,
    ) -> Result<DeviceActionResult, ControllerError> {
        self.query(|respond_to| ControllerMessage::DeviceAction {
            did,
            action,
            respond_to,
            timeout_ms,
        })
        .await
    }

    // Send a specific (typed) device action to the controller of the device.
    pub async fn device_action_inner<A: ActionTrait>(
        &self,
        did: Option<DeviceId>,
        action: A,
        timeout_ms: Option<u32>,
    ) -> Result<A::Result, ControllerError>
    // # IMPORTANT NOTE: TODO
    // The A::Result type which is returned by the action must implement the Clone trait.
    // check if A::Result can be constrained to implement Clone
    // for sure DeviceActionResultTrait cannot implement Clone as it would make it not object-safe
    //
    // Evaluate DeviceActionResultTrait { type Result: DeviceActionResultTrait + Clone }
    where
        A::Result: Clone,
    {
        let action = DeviceAction::new_inner(action);
        let result = self.device_action(did, action, timeout_ms).await?;
        match result {
            DeviceActionResult::Inner(inner) => match inner.deref().downcast_ref::<A::Result>() {
                Some(result) => Ok(result.clone()),
                None => panic!("Unexpected DeviceActionResult inner variant"),
            },
            _ => panic!("Unexpected DeviceActionResult variant"),
        }
        // Err(ControllerError::NotImplemented)
    }

    #[cfg(feature = "emu")]
    pub async fn send_emulation_request(&self, event: EmuRequest) {
        debug!("Sending emulation request to controller: {:?}", event);
        self.sender
            .send(ControllerMessage::EmulationRequest { event })
            .await
            .expect("Failed to send emulation request to controller");
    }
}
