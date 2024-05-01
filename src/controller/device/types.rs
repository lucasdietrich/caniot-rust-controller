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
