use thiserror::Error;

use crate::caniot;

use super::DeviceActionTrait;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Unsupported action for device")]
    UnsupportedAction,
    #[error("NoInnerDevice")]
    NoInnerDevice,
    #[error("Invalid frame")]
    InvalidFrame,
}

pub enum DeviceEvent<A: DeviceActionTrait> {
    Process,
    Action(A),
    Frame(caniot::ResponseData),
}
