use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::caniot::{self as ct};
use serde::{Deserialize, Serialize};

use super::{traits::ControllerAPI};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManagedDeviceError {
    #[error("Unsupported query Error")]
    UnsupportedFrame,

    #[error("Unimplemented Error")]
    NotImplemented,
}

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub command_tx: usize,
    pub attribute_write: usize,
    pub attribute_read: usize,
    pub err_rx: usize,
}

// pub struct Device<T>
// where
//     T: ManagedDeviceTrait + Send + Sync + Default + 'static,
// {
//     pub device_id: ct::DeviceId,
//     pub last_seen: Option<Instant>,
//     pub stats: DeviceStats,

//     specific: Option<T>,
// }
