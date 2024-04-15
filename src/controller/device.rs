use std::{
    fmt::Debug,
    time::{Instant},
};

use crate::caniot::{self as ct};
use serde::{Serialize};

use super::traits::ControllerAPI;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManagedDeviceError {
    #[error("Unsupported query Error")]
    UnsupportedFrame,

    #[error("Unimplemented Error")]
    NotImplemented,

    #[error("Timeout")]
    Timeout,
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

pub trait DeviceTrait: Send + Sync + 'static {
    fn handle_frame(&mut self, frame: &ct::ResponseData) -> Result<(), ManagedDeviceError>;
}

pub struct Device<T>
where
    T: DeviceTrait + Send + Sync + Default + 'static,
{
    pub device_id: ct::DeviceId,
    pub last_seen: Option<Instant>,
    pub stats: DeviceStats,

    // TODO make this private
    pub inner: T,
}

impl<T> Device<T>
where
    T: DeviceTrait + Default,
{
    pub fn new(device_id: ct::DeviceId) -> Self {
        Self {
            device_id,
            last_seen: None,
            stats: DeviceStats::default(),
            inner: T::default(),
        }
    }

    pub fn get_stats(&self) -> DeviceStats {
        self.stats
    }

    pub fn get_last_seen(&self) -> Option<Instant> {
        self.last_seen
    }

    pub fn get_device_id(&self) -> ct::DeviceId {
        self.device_id
    }

    pub async fn reset(&self, api: &mut dyn ControllerAPI) -> Result<(), ManagedDeviceError> {
        let cmd = ct::BlcCommand::get_reset_command();
        let cmd: [u8; 8] = cmd.into();

        let _z = api
            .query_command(self.device_id, ct::Endpoint::BoardControl, cmd.into(), 1000)
            .await
            .unwrap();

        Ok(())
    }
}

impl<T> DeviceTrait for Device<T>
where
    T: DeviceTrait + Send + Sync + Default + 'static,
{
    fn handle_frame(&mut self, frame: &ct::ResponseData) -> Result<(), ManagedDeviceError> {
        self.stats.rx += 1;
        self.last_seen = Some(Instant::now());
        self.inner.handle_frame(frame)
    }
}
