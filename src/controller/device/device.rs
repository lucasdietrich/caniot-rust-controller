use chrono::{DateTime, Utc};
use rocket::Request;
use serde::Serialize;
use std::time::{Duration, Instant};

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

    // Stats
    pub last_seen: Option<DateTime<Utc>>,
    pub stats: DeviceStats,

    // Inner implementation
    pub inner: Option<Box<dyn DeviceWrapperTrait>>,

    // Internal
    pub next_requested_process: Option<Instant>,
    pub last_process: Option<Instant>,
}

impl Device {
    pub fn new(did: DeviceId) -> Self {
        Self {
            did,
            last_seen: None,
            stats: DeviceStats::default(),
            inner: None,
            next_requested_process: None,
            last_process: None,
        }
    }

    pub fn mark_last_seen(&mut self) {
        self.last_seen = Some(Utc::now());
    }

    pub fn mark_processed(&mut self) {
        self.last_process = Some(Instant::now());
    }

    pub fn schedule_next_process_in(&mut self, delay: Option<Duration>) {
        if let Some(delay) = delay {
            self.next_requested_process = Some(Instant::now() + delay);
        }
    }

    pub fn needs_process(&self) -> bool {
        self.time_to_next_process()
            .and_then(|t| Some(t.as_secs() == 0))
            .unwrap_or(false)
    }

    pub fn next_process_time(&self) -> Option<Instant> {
        self.next_requested_process
    }

    pub fn time_to_next_process(&self) -> Option<Duration> {
        if self.last_process.is_none() {
            return Some(Duration::from_secs(0));
        } else if let Some(next_process) = self.next_requested_process {
            if next_process <= Instant::now() {
                return Some(Duration::from_secs(0));
            } else {
                return Some(next_process - Instant::now());
            }
        }

        None
    }
}

pub enum DeviceAction {
    Reset,
    Inner(Box<dyn DeviceActionTrait>),
}

pub enum DeviceActionResult {
    Reset,
    Inner,
    // Inner(Box<dyn DeviceActionResponseTrait>),
}

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

        if let Some(ref mut inner) = self.inner {
            inner.handle_frame(frame)
        } else {
            Ok(DeviceResult::default())
        }
    }

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        if let Some(ref mut inner) = self.inner {
            inner.process()
        } else {
            Ok(DeviceResult::default())
        }
    }
}

#[derive(Default, Debug)]
pub struct DeviceResult {
    // List of caniot requests to send to the device
    pub requests: Vec<RequestData>,

    // Time to wait before processing the device again
    pub next_process: Option<Duration>,
}

impl DeviceResult {
    pub fn request_immediate_process(&mut self) {
        self.next_process = Some(Duration::from_secs(0));
    }

    pub fn request_process_in(&mut self, delay: Duration) {
        self.next_process = Some(delay);
    }

    pub fn request_process_in_ms(&mut self, delay: u64) {
        self.request_process_in(Duration::from_millis(delay));
    }

    pub fn request_process_in_s(&mut self, delay: u64) {
        self.request_process_in(Duration::from_secs(delay));
    }

    pub fn from_request_data(request_data: RequestData) -> Self {
        let mut result: DeviceResult = Self::default();
        result.requests.push(request_data);
        result
    }
}
