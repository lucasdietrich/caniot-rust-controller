use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::caniot::{self as ct};
use serde::{Deserialize, Serialize};

use super::{traits::ControllerAPI, Unmanaged};
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

pub trait ManagedDeviceTrait: Send {
    // type Error;

    fn handle_frame(&mut self, frame: &ct::Response) -> Result<(), ManagedDeviceError> {
        Err(ManagedDeviceError::UnsupportedFrame)
    }
}

pub trait DeviceTrait: ManagedDeviceTrait {
    fn new(device_id: ct::DeviceId) -> Self;
    fn get_did(&self) -> ct::DeviceId;
    fn is_managed(&self) -> bool;
}

// pub enum Device {
//     Managed(Box<dyn ManagedDeviceTrait<Error = ManagedDeviceError>>),
//     Unmanaged(Box<dyn DeviceTrait<Error = ManagedDeviceError>>),
// }

pub struct Device<T>
where
    T: ManagedDeviceTrait + Send + Sync + Default + 'static,
{
    pub device_id: ct::DeviceId,
    pub last_seen: Option<Instant>,
    pub stats: DeviceStats,

    specific: Option<T>,
}

impl<T> ManagedDeviceTrait for Device<T>
where
    T: ManagedDeviceTrait + Send + Sync + Default + 'static,
{
    // type Error = ManagedDeviceError;

    fn handle_frame(&mut self, frame: &ct::Response) -> Result<(), ManagedDeviceError> {
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

        if let Some(managed) = &mut self.specific {
            let z = managed.handle_frame(frame);
        }

        Ok(())
    }
}

impl<T> DeviceTrait for Device<T>
where
    T: ManagedDeviceTrait + Send + Sync + Default + 'static,
{
    fn new(device_id: ct::DeviceId) -> Self {
        Device {
            device_id: device_id,
            last_seen: None,
            stats: DeviceStats::default(),
            specific: Some(T::default()),
        }
    }

    fn get_did(&self) -> ct::DeviceId {
        self.device_id
    }

    fn is_managed(&self) -> bool {
        self.specific.is_some()
    }
}

// impl<T> Device<T>
// where
//     T: ManagedDeviceTrait + Send + Sync + Default + 'static,
// {
//     pub fn new(device_id: ct::DeviceId) -> Self {
//         Device {
//             device_id: device_id,
//             last_seen: None,
//             stats: DeviceStats::default(),
//             specific: Some(T::default()),
//         }
//     }
// }

fn new_unmanaged(device_id: ct::DeviceId) -> Device<Unmanaged> {
    Device {
        device_id: device_id,
        last_seen: None,
        stats: DeviceStats::default(),
        specific: None,
    }
}