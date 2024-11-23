use thiserror::Error;

use crate::{caniot, database::SettingsError};

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Invalid frame")]
    InvalidFrame,
    #[error("NoInnerDevice")]
    NoInnerDevice,
    #[error("Unsupported action for device")]
    UnsupportedAction,
    #[error("Unsupported process type")]
    UnsupportedProcessType,
    #[error("No action result")]
    NoActionResult,
    #[error("Not implemented")]
    NotImplemented,
    #[error("Pending action still active")]
    AlreadyPendingAction,
    #[error("Protocol error")]
    ProtocolError(#[from] caniot::ProtocolError),
    #[error("Action rejected {0}")]
    ActionRejected(String),
    #[error("Settings error: {0}")]
    SettingsError(#[from] SettingsError),
}
