use tokio::sync::oneshot;

use crate::caniot::{self as ct, DeviceId};
use crate::controller::{handle::DeviceFilter, DeviceInfos};
use crate::controller::{ActionTrait, DeviceAction};
use crate::grpcserver::EmuRequest;

use super::caniot_devices_controller::ControllerError;

pub enum ControllerCaniotMessage {
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
    DevicesResetSettings {
        respond_to: oneshot::Sender<Result<(), ControllerError>>,
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
