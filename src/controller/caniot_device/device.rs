use chrono::{DateTime, Duration, Utc};

use log::warn;

use crate::{
    caniot::{
        self, classes, BoardClassTelemetry, DeviceId, Endpoint, Response, ResponseData, SysCtrl,
        TSP,
    },
    controller::{ActionTrait, DeviceAlert, JobTrait},
    utils::expirable::ExpirableTrait,
};

use super::{
    actions::{DeviceAction, DeviceActionResult},
    context::ProcessContext,
    downcast_job_as,
    traits::ActionWrapperTrait,
    verdict::{ActionVerdict, Verdict},
    DeviceControllerWrapperTrait, DeviceError, DeviceJobWrapper, DeviceJobsContext, DeviceMeasures,
    DeviceMeasuresResetJob, DeviceStats, UpdateJobVerdict,
};
#[derive(Debug)]
pub struct Device {
    pub did: DeviceId,

    // Stats
    pub last_seen: Option<DateTime<Utc>>,
    pub stats: DeviceStats,

    // Inner implementation
    pub controller: Option<Box<dyn DeviceControllerWrapperTrait>>,

    // Scheduled process
    jobs: DeviceJobsContext,

    // Last class telemetry values
    pub measures: DeviceMeasures,
}

impl Device {
    pub fn new(did: DeviceId, controller: Option<Box<dyn DeviceControllerWrapperTrait>>) -> Self {
        // TODO remove/move
        let now = Utc::now();

        Self {
            did,
            last_seen: None,
            stats: DeviceStats::default(),
            controller,
            measures: DeviceMeasures::default(),
            jobs: DeviceJobsContext::new(now),
        }
    }

    pub fn mark_last_seen(&mut self, at: DateTime<Utc>) {
        self.last_seen = Some(at);
    }

    // TODO Remove, calculate in UI
    pub fn last_seen_from_now(&self) -> Option<u32> {
        self.last_seen
            .as_ref()
            .map(|t| (Utc::now() - *t).num_seconds() as u32)
    }

    pub fn is_seen(&self) -> bool {
        self.last_seen.is_some()
    }

    /// Returns wether the inner controller can handle the action
    pub fn can_inner_controller_handle_action(&self, action: &dyn ActionWrapperTrait) -> bool {
        if let Some(inner) = self.controller.as_ref() {
            inner.wrapper_can_handle_action(action)
        } else {
            false
        }
    }

    fn handle_action_reset(&mut self) -> Result<ActionVerdict<DeviceAction>, DeviceError> {
        self.stats.reset_requested += 1;

        let req = SysCtrl::HARDWARE_RESET.into_board_request();
        Ok(ActionVerdict::ActionPendingOn(req))
    }

    fn handle_action_reset_settings(&mut self) -> Result<ActionVerdict<DeviceAction>, DeviceError> {
        self.stats.reset_settings_requested += 1;

        let req = SysCtrl::FACTORY_RESET.into_board_request();
        Ok(ActionVerdict::ActionPendingOn(req))
    }

    fn handle_action_inhibit_control(
        &mut self,
        inhibit: TSP,
    ) -> Result<ActionVerdict<DeviceAction>, DeviceError> {
        let req = SysCtrl::inhibit_control(inhibit).into_board_request();
        Ok(ActionVerdict::ActionPendingOn(req))
    }

    fn handle_action_ping(
        &mut self,
        endpoint: Endpoint,
    ) -> Result<ActionVerdict<DeviceAction>, DeviceError> {
        let req = caniot::RequestData::Telemetry { endpoint };
        Ok(ActionVerdict::ActionPendingOn(req))
    }

    pub fn reset_controller_measures_stats(&mut self) {
        self.measures.reset_minmax();
    }

    pub fn handle_action(
        &mut self,
        action: &DeviceAction,
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<DeviceAction>, DeviceError> {
        match action {
            DeviceAction::Reset => self.handle_action_reset(),
            DeviceAction::ResetSettings => self.handle_action_reset_settings(),
            DeviceAction::InhibitControl(inhibit) => self.handle_action_inhibit_control(*inhibit),
            DeviceAction::Ping(endpoint) => self.handle_action_ping(*endpoint),
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
        completed_by: Response,
    ) -> Result<<DeviceAction as ActionTrait>::Result, DeviceError> {
        match delayed_action {
            DeviceAction::Reset => Ok(DeviceActionResult::ResetSent),
            DeviceAction::ResetSettings => Ok(DeviceActionResult::ResetSettingsSent),
            DeviceAction::InhibitControl(_inhibit) => Ok(DeviceActionResult::InhibitControlSent),
            DeviceAction::Ping(_endpoint) => Ok(DeviceActionResult::Pong(completed_by)),
            DeviceAction::Inner(inner_action) => {
                if let Some(inner_device) = self.controller.as_ref() {
                    let result = inner_device
                        .wrapper_handle_delayed_action_result(inner_action, completed_by)?;
                    Ok(DeviceActionResult::new_boxed_inner(result))
                } else {
                    Err(DeviceError::NoInnerDevice)
                }
            }
        }
    }

    pub fn handle_frame(
        &mut self,
        frame: &ResponseData,
        _as_class_blc: &Option<BoardClassTelemetry>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        self.mark_last_seen(ctx.frame_received_at.unwrap());

        // Update device stats
        match frame {
            ResponseData::Telemetry { .. } => self.stats.telemetry_rx += 1,
            ResponseData::Attribute { .. } => self.stats.attribute_rx += 1,
            ResponseData::Error { .. } => self.stats.err_rx += 1,
        }

        // Ty to parse the telemetry frame as a class telemetry if possible
        let as_class_blc = match frame {
            ResponseData::Telemetry { endpoint, payload }
                if endpoint == &Endpoint::BoardControl =>
            {
                classes::telemetry::boardlc_parse_telemetry_as_class(self.did.class, payload).ok()
            }
            _ => None,
        };

        // Update the last class telemetry values
        if let Some(ref as_class_blc) = as_class_blc {
            self.measures.update_class_telemetry(as_class_blc);
        }

        // Let the inner device controller handle the frame
        if let Some(ref mut inner) = self.controller {
            inner.wrapper_handle_frame(frame, self.measures.get_class_telemetry(), ctx)
        } else {
            Ok(Verdict::default())
        }
    }

    // Process a device job
    // * Returns the result of the job processing if a job was processed
    // * Returns None if no job was processed
    pub fn process_one_job(
        &mut self,
        ctx: &mut ProcessContext,
    ) -> Option<Result<Verdict, DeviceError>> {
        if let Some(pending_job) = self.jobs.pop_pending() {
            /* Handle special jobs */
            match pending_job.definition.as_ref() {
                DeviceJobWrapper::Scheduled(ref job) => {
                    if downcast_job_as::<DeviceMeasuresResetJob>(job).is_some() {
                        self.measures.reset_minmax();
                    }
                }
                _ => {}
            };

            if let Some(ref mut inner) = self.controller {
                let result = inner.wrapper_process_one_job(
                    &pending_job.definition,
                    pending_job.timestamp,
                    ctx,
                );
                self.stats.jobs_processed += 1;
                return Some(result);
            } else {
                warn!(
                    "No controller to process job {:?} for device {}",
                    pending_job.definition, self.did
                );
            }
        }

        None
    }

    pub fn shift_jobs(&mut self, now: &DateTime<Utc>) {
        self.jobs.shift(now);
        self.stats.jobs_currently_scheduled = self.jobs.get_jobs_count();
    }

    pub fn register_new_jobs(&mut self, jobs_definitions: Vec<Box<dyn JobTrait>>) {
        self.jobs.register_new_jobs(jobs_definitions);
        self.stats.jobs_currently_scheduled = self.jobs.get_jobs_count();
    }

    pub fn update_scheduled_jobs(&mut self) {
        self.jobs.retain_jobs_definitions(|job| {
            if let Some(inner) = self.controller.as_mut() {
                match inner.wrapper_update_scheduled_job(job) {
                    UpdateJobVerdict::Keep => true,
                    UpdateJobVerdict::Unschedule => false,
                }
            } else {
                // Keep job if no defined device controller
                true
            }
        });
    }

    pub fn get_alert(&self) -> Option<DeviceAlert> {
        if !self.is_seen() {
            // TODO not fully implemented for now
            warn!("Get alert on unseen device");
            Some(DeviceAlert::new_error("Capteur non détecté"))
        } else if let Some(inner) = self.controller.as_ref() {
            inner.wrapper_get_alert()
        } else {
            None
        }
    }

    pub fn get_controller_metrics(&self) -> Vec<String> {
        if let Some(inner) = self.controller.as_ref() {
            inner.wrapper_get_metrics()
        } else {
            vec![]
        }
    }

    pub fn reset_settings(&mut self, ctx: &mut ProcessContext) {
        self.controller
            .as_mut()
            .map(|inner| inner.wrapper_reset_config(ctx));
    }
}

impl ExpirableTrait<Duration> for Device {
    const ZERO: Duration = Duration::zero();
    type Instant = DateTime<Utc>;

    fn ttl(&self, now: &DateTime<Utc>) -> Option<Duration> {
        self.jobs.ttl(now)
    }
}
