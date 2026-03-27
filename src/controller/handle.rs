use std::ops::Deref;

use as_any::Downcast;
use chrono::{DateTime, Utc};

use tokio::sync::{mpsc, oneshot};

use crate::caniot::{self as ct, DeviceId};
use serde::Serialize;

#[cfg(feature = "ble-copro")]
use super::copro_controller::{
    controller::{BleDevice, CoproControllerStats},
    sensor::BleSensor,
};

use super::{
    caniot_controller::{
        api_message::CaniotApiMessage, caniot_devices_controller::CaniotControllerError,
    },
    copro_controller::api_message::CoproApiMessage,
    filtering::{FilterCriteria, SensorFilter},
    ActionTrait, CaniotDeviceInfos, ControllerStats, DeviceAction, DeviceActionResult, DeviceStats,
    SensorAlert,
};

pub enum ControllerApiMessage {
    GetStats {
        respond_to: oneshot::Sender<ControllerStats>,
    },
    CaniotMessage(CaniotApiMessage),
    CoprocessorMessage(CoproApiMessage),
}

impl From<CaniotApiMessage> for ControllerApiMessage {
    fn from(msg: CaniotApiMessage) -> Self {
        Self::CaniotMessage(msg)
    }
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    sender: mpsc::Sender<ControllerApiMessage>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct DeviceStatsEntry {
    pub did: ct::DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub stats: DeviceStats,
}

impl ControllerHandle {
    pub fn new(sender: mpsc::Sender<ControllerApiMessage>) -> Self {
        Self { sender }
    }

    pub async fn caniot_device_request(
        &self,
        frame: ct::Request,
        timeout_ms: Option<u32>,
    ) -> Result<ct::Response, CaniotControllerError> {
        self.caniot_query(|sender| {
            CaniotApiMessage::Query {
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
    async fn caniot_query<R>(
        &self,
        build_message_closure: impl FnOnce(oneshot::Sender<R>) -> ControllerApiMessage,
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
        self.caniot_query(|respond_to| ControllerApiMessage::GetStats { respond_to })
            .await
    }

    pub async fn get_caniot_devices_infos_list(&self) -> Vec<CaniotDeviceInfos> {
        self.caniot_query(|respond_to| {
            CaniotApiMessage::GetDevices {
                filter: SensorFilter::All,
                respond_to,
            }
            .into()
        })
        .await
    }

    pub async fn get_caniot_devices_with_active_alert(&self) -> Vec<CaniotDeviceInfos> {
        self.caniot_query(|respond_to| {
            CaniotApiMessage::GetDevices {
                filter: SensorFilter::WithActiveAlert,
                respond_to,
            }
            .into()
        })
        .await
    }

    pub async fn get_caniot_device_infos(&self, did: DeviceId) -> Option<CaniotDeviceInfos> {
        self.caniot_query(|respond_to| {
            CaniotApiMessage::GetDevices {
                filter: SensorFilter::ByCriteria(FilterCriteria::CaniotId(did)),
                respond_to,
            }
            .into()
        })
        .await
        .into_iter()
        .next()
    }

    pub async fn get_caniot_device_infos_by_filter(
        &self,
        filter: SensorFilter,
    ) -> Option<CaniotDeviceInfos> {
        self.caniot_query(|respond_to| CaniotApiMessage::GetDevices { filter, respond_to }.into())
            .await
            .into_iter()
            .next()
    }

    // Send a generic device action to the controller of the device.
    // Some actions are unique to a specific device, so the device id is optional.
    // For generic actions, the device id is required.
    pub async fn caniot_device_action(
        &self,
        did: Option<DeviceId>,
        action: DeviceAction,
        timeout_ms: Option<u32>,
    ) -> Result<DeviceActionResult, CaniotControllerError> {
        self.caniot_query(|respond_to| {
            CaniotApiMessage::DeviceAction {
                did,
                action,
                respond_to: Some(respond_to),
                timeout_ms,
            }
            .into()
        })
        .await
    }

    pub async fn caniot_device_action_no_response(
        &self,
        did: Option<DeviceId>,
        action: DeviceAction,
    ) {
        let message: CaniotApiMessage = CaniotApiMessage::DeviceAction {
            did,
            action,
            respond_to: None,
            timeout_ms: None,
        };
        self.sender
            .send(message.into())
            .await
            .expect("Failed to send IPC message to controller");
    }

    pub async fn caniot_reset_devices_measures_stats(&self) {
        self.sender
            .send(CaniotApiMessage::DevicesResetMeasuresStats.into())
            .await
            .expect("Failed to send IPC message to controller");
    }

    // Send a specific (typed) device action to the controller of the device.
    pub async fn caniot_device_action_inner<A: ActionTrait>(
        &self,
        did: Option<DeviceId>,
        action: A,
        timeout_ms: Option<u32>,
    ) -> Result<A::Result, CaniotControllerError>
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
        let result = self.caniot_device_action(did, action, timeout_ms).await?;
        match result {
            DeviceActionResult::Inner(inner) => match inner.deref().downcast_ref::<A::Result>() {
                Some(result) => Ok(result.clone()),
                None => panic!("Unexpected DeviceActionResult inner variant"),
            },
            _ => panic!("Unexpected DeviceActionResult variant"),
        }
        // Err(ControllerError::NotImplemented)
    }

    // IS THIS FUNCTION OK ??
    pub async fn caniot_device_action_inner_no_response<A: ActionTrait>(
        &self,
        did: Option<DeviceId>,
        action: A,
    ) {
        self.caniot_device_action_no_response(did, DeviceAction::new_inner(action))
            .await;
    }

    pub async fn reset_caniot_devices_settings(&self) -> Result<(), CaniotControllerError> {
        self.caniot_query(|respond_to| CaniotApiMessage::DevicesResetSettings { respond_to }.into())
            .await
    }

    #[cfg(feature = "ble-copro")]
    pub async fn get_copro_devices_list(&self) -> Vec<BleSensor> {
        self.get_copro_devices_by_filter(SensorFilter::All).await
    }

    #[cfg(feature = "ble-copro")]
    pub async fn get_copro_devices_by_filter(&self, filter: SensorFilter) -> Vec<BleSensor> {
        let (respond_to, receiver) = oneshot::channel();
        let message = ControllerApiMessage::CoprocessorMessage(CoproApiMessage::GetDevices {
            filter,
            respond_to,
        });
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
        receiver.await.expect("IPC Sender dropped before response")
    }

    #[cfg(feature = "ble-copro")]
    pub async fn get_copro_alert(&self) -> Option<SensorAlert> {
        let (respond_to, receiver) = oneshot::channel();
        let message =
            ControllerApiMessage::CoprocessorMessage(CoproApiMessage::GetAlert { respond_to });
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
        receiver.await.expect("IPC Sender dropped before response")
    }

    #[cfg(feature = "ble-copro")]
    pub async fn get_copro_controller_stats(&self) -> CoproControllerStats {
        let (respond_to, receiver) = oneshot::channel();
        let message =
            ControllerApiMessage::CoprocessorMessage(CoproApiMessage::GetStats { respond_to });
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
        receiver.await.expect("IPC Sender dropped before response")
    }

    #[cfg(feature = "ble-copro")]
    pub async fn reset_copro_devices_measures_stats(&self) {
        let message =
            ControllerApiMessage::CoprocessorMessage(CoproApiMessage::ResetDevicesMeasuresStats);
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
    }

    #[cfg(feature = "ble-copro")]
    pub async fn get_ble_devices_state(&self) -> Vec<BleDevice> {
        let (respond_to, receiver) = oneshot::channel();
        let message =
            ControllerApiMessage::CoprocessorMessage(CoproApiMessage::GetBleDevicesState {
                respond_to,
            });
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
        receiver.await.expect("IPC Sender dropped before response")
    }

    #[cfg(feature = "ble-copro")]
    pub async fn ble_remove_bonds(&self) {
        let message = ControllerApiMessage::CoprocessorMessage(CoproApiMessage::BleRemoveBonds);
        self.sender
            .send(message)
            .await
            .expect("Failed to send IPC message to controller");
    }
}
