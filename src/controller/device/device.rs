use as_any::{AsAny, Downcast};
use chrono::{DateTime, Utc};
use rocket::Request;
use serde::Serialize;
use std::{
    ops::Deref,
    time::{Duration, Instant},
};

use crate::{
    caniot::{
        self, emu::device, Action, BlcCommand, BlcPayload, DeviceId, RequestData, Response,
        ResponseData,
    },
    controller::{DeviceActionResultTrait, DeviceActionTrait, DeviceActionWrapperTrait},
};

use super::{
    actions::{DeviceAction, DeviceActionResult},
    DeviceError, DeviceTrait, DeviceWrapperTrait,
};

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
    // Strategies (e.g. for retries)
    // pub strategies: Vec<Box<dyn DeviceStrategy>>,
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

impl DeviceTrait for Device {
    type Action = DeviceAction;

    fn handle_action(
        &mut self,
        action: &DeviceAction,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        match action {
            DeviceAction::Reset => Ok(DeviceProcessOutput::default()), // BlcCommand::HARDWARE_RESET

            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.inner.as_mut() {
                    let inner_result = inner_device.handle_action(inner_action)?;
                    Ok(DeviceProcessOutput::from_inner_result(inner_result))
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
        }
    }

    fn handle_frame(
        &mut self,
        frame: &ResponseData,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        self.mark_last_seen();

        // update stats
        match frame {
            ResponseData::Telemetry { .. } => self.stats.telemetry_rx += 1,
            ResponseData::Attribute { .. } => self.stats.attribute_rx += 1,
            ResponseData::Error { .. } => self.stats.err_rx += 1,
        }

        if let Some(ref mut inner) = self.inner {
            let inner_result = inner.handle_frame(frame)?;
            Ok(DeviceProcessOutput::from_inner_result(inner_result))
        } else {
            Ok(DeviceProcessOutput::default())
        }
    }

    fn process(&mut self) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        if let Some(ref mut inner) = self.inner {
            let inner_result = inner.process()?;
            Ok(DeviceProcessOutput::from_inner_result(inner_result))
        } else {
            Ok(DeviceProcessOutput::default())
        }
    }
}

#[derive(Debug)]
pub struct DeviceProcessOutput<A: DeviceActionTrait> {
    // List of caniot requests to send to the device
    pub requests: Vec<RequestData>,

    // Time to wait before processing the device again
    pub next_process: Option<Duration>,

    // Action result
    pub action_result: Option<A::Result>,
}

impl<A: DeviceActionTrait> Default for DeviceProcessOutput<A> {
    fn default() -> Self {
        Self {
            requests: Vec::new(),
            next_process: None,
            action_result: None,
        }
    }
}

impl<A: DeviceActionTrait> DeviceProcessOutput<A> {
    pub fn request_process_in(&mut self, delay: Duration) {
        self.next_process = Some(delay);
    }

    pub fn request_process_in_ms(&mut self, delay: u64) {
        self.request_process_in(Duration::from_millis(delay));
    }

    pub fn request_process_in_s(&mut self, delay: u64) {
        self.request_process_in(Duration::from_secs(delay));
    }

    pub fn request_process_immediate(&mut self) {
        self.request_process_in_ms(0);
    }

    pub fn build_request_data(request_data: RequestData) -> Self {
        let mut result = DeviceProcessOutput::<A>::default();
        result.requests.push(request_data);
        result
    }

    pub fn build_action_result(action_result: A::Result) -> Self {
        let mut result = DeviceProcessOutput::<A>::default();
        result.action_result = Some(action_result);
        result
    }
}

impl DeviceProcessOutput<DeviceAction> {
    /// Converts a DeviceProcessOutputWrapper returned by an inner device to a DeviceProcessOutput<DeviceAction>
    pub fn from_inner_result(inner: DeviceProcessOutputWrapper) -> Self {
        DeviceProcessOutput {
            requests: inner.requests,
            next_process: inner.next_process,
            action_result: inner.action_result.map(DeviceActionResult::new_boxed_inner),
        }
    }
}

impl<A> DeviceActionResultTrait for DeviceProcessOutput<A> where A: DeviceActionTrait {}

pub struct DeviceProcessOutputWrapper {
    pub requests: Vec<RequestData>,
    pub next_process: Option<Duration>,
    pub action_result: Option<Box<dyn DeviceActionResultTrait>>,
}

impl<A> From<DeviceProcessOutput<A>> for DeviceProcessOutputWrapper
where
    A: DeviceActionTrait,
{
    fn from(result: DeviceProcessOutput<A>) -> Self {
        Self {
            requests: result.requests,
            next_process: result.next_process,
            action_result: result
                .action_result
                .map(|r| Box::new(r) as Box<dyn DeviceActionResultTrait>),
        }
    }
}
