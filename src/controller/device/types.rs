use thiserror::Error;

use crate::caniot;

use super::DeviceActionTrait;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Invalid frame")]
    InvalidFrame,
    #[error("NoInnerDevice")]
    NoInnerDevice,
    #[error("Unsupported action for device")]
    UnsupportedAction,
    #[error("No action result")]
    NoActionResult,
    #[error("Not implemented")]
    NotImplemented,
    #[error("Pending action still active")]
    AlreadyPendingAction,
    #[error("Protocol error")]
    ProtocolError(#[from] caniot::ProtocolError),
}

pub enum DeviceEvent<A: DeviceActionTrait> {
    // Device controller is called without any specific event
    Process,

    // Device controller is called with an action event
    Action(A),

    // // Device controller is called and should provide an action result
    // ActionResult(A),

    // Device controller is called with a device frame
    Frame(caniot::ResponseData),

    // Device controller is called with a device frame as a response to a specific action
    FrameForAction(caniot::ResponseData, A),
}
