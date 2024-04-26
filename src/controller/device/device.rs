use chrono::{DateTime, Utc};
use rocket::Request;
use serde::Serialize;
use std::time::Instant;

use crate::{
    caniot::{self, BlcCommand, BlcPayload, DeviceId, RequestData, Response, ResponseData},
    controller::DeviceActionTrait,
};

use super::{DeviceError, DeviceTrait, DeviceWrapperTrait};

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub command_tx: usize,
    pub attribute_rx: usize,
    pub attribute_tx: usize,
    pub err_rx: usize,
}

#[derive(Debug)]
pub struct Device {
    pub did: DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub stats: DeviceStats,

    pub inner: Option<Box<dyn DeviceWrapperTrait>>,
}

impl Device {
    pub fn new(did: DeviceId) -> Self {
        Self {
            did,
            last_seen: None,
            stats: DeviceStats::default(),
            inner: None,
        }
    }

    pub fn mark_last_seen(&mut self) {
        self.last_seen = Some(Utc::now());
    }
}

pub enum DeviceAction {
    Reset,
    Inner(Box<dyn DeviceActionTrait>),
}

impl DeviceActionTrait for DeviceAction {}

impl DeviceTrait for Device {
    type Action = DeviceAction;

    fn handle_action(&mut self, action: &DeviceAction) -> Result<DeviceResult, DeviceError> {
        match action {
            DeviceAction::Reset => Ok(DeviceResult::default()), // BlcCommand::HARDWARE_RESET

            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.inner.as_mut() {
                    inner_device.handle_action(inner_action)
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
        }
    }

    fn handle_frame(&mut self, frame: &ResponseData) -> Result<DeviceResult, DeviceError> {
        self.mark_last_seen();

        // update stats
        match frame {
            ResponseData::Telemetry { .. } => self.stats.telemetry_rx += 1,
            ResponseData::Attribute { .. } => self.stats.attribute_rx += 1,
            ResponseData::Error { .. } => self.stats.err_rx += 1,
        }

        Ok(DeviceResult::default())
    }

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        Ok(DeviceResult::default())
    }
}

#[derive(Default, Debug)]
pub struct DeviceResult {
    pub requests: Vec<RequestData>,
}
