use chrono::{DateTime, Utc};

use serde::Serialize;
use std::time::{Duration, Instant};

use crate::{
    caniot::{
        utils::blc_parse_telemetry_as_class, BlcClassTelemetry, DeviceId, Endpoint, ResponseData,
    },
    controller::ActionTrait,
};

use super::{
    actions::{DeviceAction, DeviceActionResult},
    context::ProcessContext,
    traits::ActionWrapperTrait,
    verdict::{ActionVerdict, Verdict},
    DeviceControllerTrait, DeviceControllerWrapperTrait, DeviceError,
};

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub telemetry_tx: usize,
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
    pub controller: Option<Box<dyn DeviceControllerWrapperTrait>>,

    // Internal
    pub next_requested_process: Option<Instant>,
    pub last_process: Option<Instant>,

    // Last class telemetry values
    pub measures: Option<BlcClassTelemetry>,
}

impl Device {
    pub fn new(did: DeviceId) -> Self {
        Self {
            did,
            last_seen: None,
            stats: DeviceStats::default(),
            controller: None,
            next_requested_process: None,
            last_process: None,
            measures: None,
        }
    }

    pub fn mark_last_seen(&mut self) {
        self.last_seen = Some(Utc::now());
    }

    pub fn last_seen_from_now(&self) -> Option<u32> {
        self.last_seen
            .as_ref()
            .map(|t| (Utc::now() - *t).num_seconds() as u32)
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

    pub fn can_inner_controller_handle_action(&self, action: &dyn ActionWrapperTrait) -> bool {
        if let Some(inner) = self.controller.as_ref() {
            inner.wrapper_can_handle_action(action)
        } else {
            false
        }
    }

    pub fn handle_action(
        &mut self,
        action: &DeviceAction,
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<DeviceAction>, DeviceError> {
        match action {
            DeviceAction::Reset => Err(DeviceError::NotImplemented), // BlcCommand::HARDWARE_RESET

            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.controller.as_mut() {
                    let inner_verdict = inner_device.wrapper_handle_action(inner_action, ctx)?;
                    Ok(ActionVerdict::from_inner_verdict(inner_verdict))
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
        }
    }

    pub fn handle_action_result(
        &self,
        delayed_action: &DeviceAction,
    ) -> Result<<DeviceAction as ActionTrait>::Result, DeviceError> {
        match delayed_action {
            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.controller.as_ref() {
                    let result = inner_device.wrapper_handle_delayed_action_result(inner_action)?;
                    Ok(DeviceActionResult::new_boxed_inner(result))
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
            _ => Err(DeviceError::NotImplemented),
        }
    }

    pub fn handle_frame(
        &mut self,
        frame: &ResponseData,
        _as_class_blc: &Option<BlcClassTelemetry>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        self.mark_last_seen();

        // Update device stats
        match frame {
            ResponseData::Telemetry { .. } => self.stats.telemetry_rx += 1,
            ResponseData::Attribute { .. } => self.stats.attribute_rx += 1,
            ResponseData::Error { .. } => self.stats.err_rx += 1,
        }

        // Try to parse the telemetry frame as a class telemetry if possible
        let as_class_blc = match frame {
            ResponseData::Telemetry { endpoint, payload }
                if endpoint == &Endpoint::BoardControl =>
            {
                blc_parse_telemetry_as_class(self.did.class, payload).ok()
            }
            _ => None,
        };

        // Update the last class telemetry values
        if let Some(as_class_blc) = as_class_blc {
            self.measures = Some(as_class_blc);
        }

        // Let the inner device controller handle the frame
        if let Some(ref mut inner) = self.controller {
            inner.wrapper_handle_frame(frame, &self.measures, ctx)
        } else {
            Ok(Verdict::default())
        }
    }

    pub fn process(&mut self, ctx: &mut ProcessContext) -> Result<Verdict, DeviceError> {
        if let Some(ref mut inner) = self.controller {
            inner.wrapper_process(ctx)
        } else {
            Ok(Verdict::default())
        }
    }
}
