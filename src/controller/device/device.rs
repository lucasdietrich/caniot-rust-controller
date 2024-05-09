
use chrono::{DateTime, Utc};

use serde::Serialize;
use std::{
    time::{Duration, Instant},
};

use crate::{
    caniot::{
        DeviceId,
        ResponseData,
    },
    controller::{ActionTrait},
};

use super::{
    actions::{DeviceAction, DeviceActionResult},
    context::ProcessContext,
    verdict::{ActionVerdict, Verdict},
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
            // pending_action: None,
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
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<Self::Action>, DeviceError> {
        match action {
            DeviceAction::Reset => Err(DeviceError::NotImplemented), // BlcCommand::HARDWARE_RESET

            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.inner.as_mut() {
                    let inner_verdict = inner_device.wrapper_handle_action(inner_action, ctx)?;
                    Ok(ActionVerdict::from_inner_verdict(inner_verdict))
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
        }
    }

    fn handle_action_result(
        &self,
        delayed_action: &Self::Action,
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        match delayed_action {
            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.inner.as_ref() {
                    let result = inner_device.wrapper_handle_delayed_action_result(inner_action)?;
                    Ok(DeviceActionResult::new_boxed_inner(result))
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
            _ => Err(DeviceError::NotImplemented),
        }
    }

    fn handle_frame(
        &mut self,
        frame: &ResponseData,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        self.mark_last_seen();

        // Update device stats
        match frame {
            ResponseData::Telemetry { .. } => self.stats.telemetry_rx += 1,
            ResponseData::Attribute { .. } => self.stats.attribute_rx += 1,
            ResponseData::Error { .. } => self.stats.err_rx += 1,
        }

        if let Some(ref mut inner) = self.inner {
            inner.wrapper_handle_frame(frame, ctx)
        } else {
            Ok(Verdict::default())
        }
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> Result<Verdict, DeviceError> {
        if let Some(ref mut inner) = self.inner {
            inner.wrapper_process(ctx)
        } else {
            Ok(Verdict::default())
        }
    }
}
