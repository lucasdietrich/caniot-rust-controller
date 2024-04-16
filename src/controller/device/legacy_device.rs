use std::{fmt::Debug, time::Instant};

use crate::caniot::{self as ct};
use serde::Serialize;

use super::super::ControllerAPI;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LManagedDeviceError {
    #[error("Unsupported query Error")]
    UnsupportedFrame,

    #[error("Unimplemented Error")]
    NotImplemented,

    #[error("Timeout")]
    Timeout,
}

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct LDeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub command_tx: usize,
    pub attribute_write: usize,
    pub attribute_read: usize,
    pub err_rx: usize,
}

pub trait LDeviceTrait: Send + Sync + 'static {
    fn handle_frame(&mut self, frame: &ct::ResponseData) -> Result<(), LManagedDeviceError>;
}

pub struct LDevice<T>
where
    T: LDeviceTrait + Send + Sync + Default + 'static,
{
    pub device_id: ct::DeviceId,
    pub last_seen: Option<Instant>,
    pub stats: LDeviceStats,

    // TODO make this private
    pub inner: T,
}

impl<T> LDevice<T>
where
    T: LDeviceTrait + Default,
{
    pub fn new(device_id: ct::DeviceId) -> Self {
        Self {
            device_id,
            last_seen: None,
            stats: LDeviceStats::default(),
            inner: T::default(),
        }
    }

    pub fn get_stats(&self) -> LDeviceStats {
        self.stats
    }

    pub fn get_last_seen(&self) -> Option<Instant> {
        self.last_seen
    }

    pub fn get_device_id(&self) -> ct::DeviceId {
        self.device_id
    }

    pub async fn reset(&self, api: &mut dyn ControllerAPI) -> Result<(), LManagedDeviceError> {
        let cmd = ct::BlcCommand::get_reset_command();
        let cmd: [u8; 8] = cmd.into();

        let _z = api
            .query_command(self.device_id, ct::Endpoint::BoardControl, cmd.into(), 1000)
            .await
            .unwrap();

        Ok(())
    }
}

impl<T> LDeviceTrait for LDevice<T>
where
    T: LDeviceTrait + Send + Sync + Default + 'static,
{
    fn handle_frame(&mut self, frame: &ct::ResponseData) -> Result<(), LManagedDeviceError> {
        self.stats.rx += 1;
        self.last_seen = Some(Instant::now());
        self.inner.handle_frame(frame)
    }
}
