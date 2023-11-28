use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::caniot as ct;
use serde::{Deserialize, Serialize};

use super::traits::ControllerAPI;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Unsupported query Error")]
    UnsupportedFrame,
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

pub struct Device<T>
where
    T: DeviceTrait + Send + Sync + 'static,
{
    pub device_id: ct::DeviceId,
    pub last_seen: Option<Instant>,
    pub stats: DeviceStats,

    specific: T,
}

pub trait DeviceTrait {
    type Error;

    fn handler_frame(&mut self, frame: &ct::Response) -> Result<(), Self::Error>;
}

impl<T> DeviceTrait for Device<T>
where
    T: DeviceTrait + Send + Sync + 'static,
{
    type Error = DeviceError;

    fn handler_frame(&mut self, frame: &ct::Response) -> Result<(), DeviceError> {
        match frame.data {
            ct::ResponseData::Attribute { .. } => {
                self.stats.attribute_read += 1;
            }
            ct::ResponseData::Telemetry { .. } => {
                self.stats.telemetry_rx += 1;
            }
            ct::ResponseData::Error { .. } => {
                self.stats.err_rx += 1;
            }
        }

        self.last_seen = Some(std::time::Instant::now());

        let z = self.specific.handler_frame(frame);

        Ok(())
    }
}
