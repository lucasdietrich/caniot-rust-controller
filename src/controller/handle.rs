use std::{cmp::Ordering, ops::Deref};

use as_any::Downcast;
use chrono::{DateTime, Utc};

use tokio::sync::{mpsc, oneshot};

#[cfg(feature = "emu")]
use crate::grpcserver::EmuRequest;
use crate::{
    bus::CanStats,
    caniot::{self as ct, DeviceId},
};
use serde::Serialize;

use super::{
    alert,
    caniot_controller::{
        caniot_devices_controller::ControllerError, caniot_message::ControllerCaniotMessage,
    },
    controller::controller::ControllerCoreStats,
    ActionTrait, CaniotControllerStats, ControllerStats, Device, DeviceAction, DeviceActionResult,
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
    GetStats {
        respond_to: oneshot::Sender<ControllerStats>,
    },
    CaniotMessage(ControllerCaniotMessage),
    CoprocessorMessage,
}

impl From<ControllerCaniotMessage> for ControllerMessage {
    fn from(msg: ControllerCaniotMessage) -> Self {
        Self::CaniotMessage(msg)
    }
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
        self.query(|sender| {
            ControllerCaniotMessage::Query {
                query: frame,
                timeout_ms,
                respond_to: Some(sender),
            }
            .into()
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

    pub async fn get_controller_stats(&self) -> ControllerStats {
        self.query(|respond_to| ControllerMessage::GetStats { respond_to })
            .await
    }

    pub async fn get_devices_infos_list(&self) -> Vec<DeviceInfos> {
        self.query(|respond_to| {
            ControllerCaniotMessage::GetDevices {
                filter: DeviceFilter::All,
                respond_to,
            }
            .into()
        })
        .await
    }

    pub async fn get_devices_with_active_alert(&self) -> Vec<DeviceInfos> {
        self.query(|respond_to| {
            ControllerCaniotMessage::GetDevices {
                filter: DeviceFilter::WithActiveAlert,
                respond_to,
            }
            .into()
        })
        .await
    }

    pub async fn get_device_infos(&self, did: DeviceId) -> Option<DeviceInfos> {
        self.query(|respond_to| {
            ControllerCaniotMessage::GetDevices {
                filter: DeviceFilter::ById(did),
                respond_to,
            }
            .into()
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
        self.query(|respond_to| {
            ControllerCaniotMessage::DeviceAction {
                did,
                action,
                respond_to,
                timeout_ms,
            }
            .into()
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
            .send(ControllerCaniotMessage::EmulationRequest { event }.into())
            .await
            .expect("Failed to send emulation request to controller");
    }

    pub async fn reset_devices_settings(&self) -> Result<(), ControllerError> {
        self.query(|respond_to| ControllerCaniotMessage::DevicesResetSettings { respond_to }.into())
            .await
    }
}
