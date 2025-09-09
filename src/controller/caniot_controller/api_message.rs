use tokio::sync::oneshot;

use crate::caniot::{self as ct, DeviceId};
use crate::controller::device_filtering::DeviceFilter;
use crate::controller::CaniotDeviceInfos;
use crate::controller::{ActionTrait, DeviceAction};

use super::caniot_devices_controller::CaniotControllerError;

pub enum CaniotApiMessage {
    GetDevices {
        filter: DeviceFilter,
        respond_to: oneshot::Sender<Vec<CaniotDeviceInfos>>,
    },
    Query {
        query: ct::Request,
        timeout_ms: Option<u32>,

        // If Some, the controller will respond to the sender with the result of the query.
        // If None, the controller will send the query and not wait for a response.
        respond_to: Option<oneshot::Sender<Result<ct::Response, CaniotControllerError>>>,
    },
    DevicesResetMeasuresStats,
    DeviceAction {
        did: Option<DeviceId>,
        action: DeviceAction,
        respond_to:
            oneshot::Sender<Result<<DeviceAction as ActionTrait>::Result, CaniotControllerError>>,
        timeout_ms: Option<u32>,
    },
    DevicesResetSettings {
        respond_to: oneshot::Sender<Result<(), CaniotControllerError>>,
    },
}
