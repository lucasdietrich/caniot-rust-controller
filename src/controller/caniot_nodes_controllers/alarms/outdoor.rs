use chrono::{DateTime, Duration, Utc};
use log::{info, warn};
use serde::{Deserialize, Serialize};

use super::{
    actions::{Action, AlarmEnable},
    config::{AlarmConfig, ConfigUpdateAutoJobsResult},
    jobs::{AlarmJob, AutoAction, AutoDevice},
    AlarmPartialConfig,
};
use crate::{
    caniot::{self, RequestData, Response, Xps},
    controller::{
        alarms::{actions::SirenAction, types::OutdoorAlarmCommand},
        ActionResultTrait, ActionTrait, ActionVerdict, ConfigTrait, DeviceAlert,
        DeviceControllerInfos, DeviceControllerTrait, DeviceError, DeviceJobImpl,
        PartialConfigTrait, ProcessContext, UpdateJobVerdict, Verdict,
    },
    ha::LOCATION_OUTSIDE,
    utils::{
        format_metric,
        monitorable_state::{MonitorableResultTrait, StateMonitor},
        SensorLabel,
    },
};

#[derive(Debug, Clone, Default)]
pub struct AlarmContext {
    pub state: StateMonitor<AlarmEnable>,

    pub last_siren_activation: Option<DateTime<Utc>>,

    pub sabotage: StateMonitor<bool>,
}

impl AlarmContext {
    pub fn set_enable(&mut self, state: &AlarmEnable) -> Option<AlarmEnable> {
        self.state.update(state.clone())
    }

    pub fn is_armed(&self) -> bool {
        matches!(self.state.as_ref(), AlarmEnable::Armed)
    }
}

impl MonitorableResultTrait for Option<AlarmEnable> {
    fn has_changed(&self) -> bool {
        self.is_some()
    }

    fn is_falling(&self) -> bool {
        matches!(self, Some(AlarmEnable::Armed))
    }

    fn is_rising(&self) -> bool {
        matches!(self, Some(AlarmEnable::Disarmed))
    }
}

#[derive(Debug, Clone)]
pub struct NightLightsContext {
    // Auto mode is enabled
    // Lights turn on when presence is detected
    pub auto_active: bool,

    // Desired duration for the lights to stay on when presence is detected
    pub desired_duration: Duration,
}

impl Default for NightLightsContext {
    fn default() -> Self {
        Self {
            // Auto mode is enabled by default
            auto_active: false,
            desired_duration: Duration::seconds(60),
        }
    }
}

impl NightLightsContext {
    #[allow(dead_code)]
    pub fn set_auto_active(&mut self, state: bool) {
        self.auto_active = state;
    }

    pub fn is_active(&self) -> bool {
        self.auto_active
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeviceIOState {
    pub siren: bool,          // true if siren is on
    pub detectors: [bool; 2], // east, south (true if presence detected)
    pub lights: [bool; 2],    // south, east (true if lights are on)
    pub sabotage: bool,       // false if sabotage detected
}

impl DeviceIOState {
    pub fn is_siren_on(&self) -> bool {
        self.siren
    }

    pub fn get_south_detector(&self) -> bool {
        self.detectors[1]
    }

    pub fn get_east_detector(&self) -> bool {
        self.detectors[0]
    }

    pub fn get_sabotage(&self) -> bool {
        self.sabotage
    }

    pub fn get_south_light(&self) -> bool {
        self.lights[0]
    }

    pub fn get_east_light(&self) -> bool {
        self.lights[1]
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct AlarmStats {
    pub south_detector_triggered_count: u32,
    pub east_detector_triggered_count: u32,
    pub sabotage_triggered_count: u32,
    pub sirens_triggered_count: u32,
    pub last_event: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default)]
pub struct AlarmController {
    pub ios: DeviceIOState,
    // ios state

    // general state
    pub south_detector: StateMonitor<bool>,
    pub east_detector: StateMonitor<bool>,

    // internal state
    pub alarm: AlarmContext,
    pub night_lights: NightLightsContext,

    // stats
    pub stats: AlarmStats,

    // config
    pub config: AlarmConfig,
}

#[derive(Debug, Clone)]
pub struct AlarmControllerReport {
    // ios state
    pub ios: DeviceIOState,

    // internal state
    pub alarm_enabled: bool,
    pub last_siren_activation: Option<DateTime<Utc>>,

    // stats
    pub stats: AlarmStats,

    // config
    pub config: AlarmConfig,
}

impl ActionResultTrait for AlarmControllerReport {}

impl AlarmController {
    // Returns whether the minimum interval between two siren activations has passed
    pub fn is_siren_interval_passed(&self, now: &DateTime<Utc>) -> bool {
        match self.alarm.last_siren_activation {
            Some(last_activation) => {
                let interval =
                    Duration::seconds(self.config.alarm_siren_minimum_interval_seconds as i64);
                *now - last_activation >= interval
            }
            None => true,
        }
    }

    /// Updates the device state with a new state provided as input.
    ///
    /// Arguments:
    ///
    /// * `new_state`: New state to update the device with.
    ///
    /// Returns:
    ///
    /// Returns `request_data` if a request should be sent to the device.
    pub fn update_state(
        &mut self,
        new_state: DeviceIOState,
        now: &DateTime<Utc>,
    ) -> Option<RequestData> {
        self.ios = new_state;

        let mut command = OutdoorAlarmCommand::default();

        let (south_detector_result, east_detector_result, sabotage_result) = (
            self.east_detector.update(self.ios.get_east_detector()),
            self.south_detector.update(self.ios.get_south_detector()),
            self.alarm.sabotage.update(self.ios.get_sabotage()),
        );

        let mut detector_triggered = false;
        if south_detector_result.is_rising() {
            detector_triggered = true;
            self.stats.south_detector_triggered_count += 1;
            self.stats.last_event = Some(*now);
        }

        if east_detector_result.is_rising() {
            detector_triggered = true;
            self.stats.east_detector_triggered_count += 1;
            self.stats.last_event = Some(*now);
        }

        let mut trigger_siren = false;
        if detector_triggered {
            info!("Presence detected");
            if self.night_lights.is_active() {
                info!("Lights turned on");
                command.set_east_light(Xps::PulseOn);
                command.set_south_light(Xps::PulseOn);
            }
            if self.alarm.is_armed() {
                warn!("Presence detected while alarm is armed, activating siren");
                trigger_siren = true;
            }
        }

        if sabotage_result.is_rising() {
            self.stats.sabotage_triggered_count += 1;
            self.stats.last_event = Some(*now);
            warn!("Sabotage detected on the outdoor alarm");
            if self.alarm.is_armed() {
                warn!("Sabotage detected while alarm is armed, activating siren");
                trigger_siren = true;
            }
        }

        if trigger_siren {
            if self.is_siren_interval_passed(now) {
                command.set_siren(Xps::PulseOn);
                command.set_east_light(Xps::PulseOn);
                command.set_south_light(Xps::PulseOn);
                self.stats.sirens_triggered_count += 1;
                self.alarm.last_siren_activation = Some(*now);
            } else {
                warn!("Siren activation blocked by minimum interval");
            }
        }

        command.has_effect().then(|| command.into_request())
    }

    /// Returns the current state of the device.
    pub fn get_state(&self) -> AlarmControllerReport {
        AlarmControllerReport {
            ios: self.ios.clone(),
            alarm_enabled: self.alarm.is_armed(),
            last_siren_activation: self.alarm.last_siren_activation,
            stats: self.stats.clone(),
            config: self.config.clone(),
        }
    }
}

impl DeviceControllerTrait for AlarmController {
    type Action = Action;
    type Job = AlarmJob;
    type Config = AlarmConfig;

    fn new(config: Option<&Self::Config>) -> Self {
        Self {
            config: config.cloned().unwrap_or_default(),
            ..Default::default()
        }
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }

    fn patch_config(
        &mut self,
        partial: AlarmPartialConfig,
        ctx: &mut ProcessContext,
    ) -> Result<(), DeviceError> {
        let patch_result = self.config.patch(partial.clone())?;

        fn handle_update_result(
            update_result: ConfigUpdateAutoJobsResult,
            ctx: &mut ProcessContext,
        ) {
            match update_result {
                ConfigUpdateAutoJobsResult::EnableJobs(enable_job, disable_job) => {
                    ctx.add_job(enable_job);
                    ctx.add_job(disable_job);
                }
                ConfigUpdateAutoJobsResult::UpdateJobs => {
                    ctx.request_jobs_update();
                }
                ConfigUpdateAutoJobsResult::Unchanged => {}
            }
        }

        handle_update_result(patch_result.alarm, ctx);
        handle_update_result(patch_result.lights, ctx);

        // Set the update future
        let storage = ctx.storage.clone();
        ctx.set_async_future(async move { Ok(partial.save(&storage.get_settings_store()).await?) });

        Ok(())
    }

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new(
            "outdoor_alarm",
            Some(LOCATION_OUTSIDE),
            Some("Controleur d'alarme extérieure"),
            Some("alarms"),
        )
    }

    fn get_alert(&self) -> Option<DeviceAlert> {
        if self.ios.is_siren_on() {
            Some(DeviceAlert::new_warning(
                "Sirene d'alarme extérieure active",
            ))
        } else if *self.alarm.sabotage {
            Some(DeviceAlert::new_error(
                "Sabotage d'alarme (extérieure) détecté",
            ))
        } else if self.alarm.is_armed() {
            Some(DeviceAlert::new_ok("Alarme extérieure active"))
        } else {
            None
        }
    }

    fn process_job(
        &mut self,
        job: &DeviceJobImpl<Self::Job>,
        _job_timestamp: DateTime<Utc>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        match job {
            // Declare jobs that should be executed when the device is added
            DeviceJobImpl::DeviceAdd => {
                if self.config.auto_alarm_enable {
                    ctx.add_job(AlarmJob::DailyAuto(
                        self.config.auto_alarm_enable_time,
                        AutoDevice::Alarm,
                        AutoAction::Enable,
                    ));
                    ctx.add_job(AlarmJob::DailyAuto(
                        self.config.auto_alarm_disable_time,
                        AutoDevice::Alarm,
                        AutoAction::Disable,
                    ));
                }

                if self.config.auto_lights_enable {
                    ctx.add_job(AlarmJob::DailyAuto(
                        self.config.auto_lights_enable_time,
                        AutoDevice::Lights,
                        AutoAction::Enable,
                    ));
                    ctx.add_job(AlarmJob::DailyAuto(
                        self.config.auto_lights_disable_time,
                        AutoDevice::Lights,
                        AutoAction::Disable,
                    ));
                }
            }
            DeviceJobImpl::Scheduled(job) => match job {
                AlarmJob::DailyAuto(_, AutoDevice::Alarm, alarm_action) => {
                    let action = match alarm_action {
                        AutoAction::Enable => AlarmEnable::Armed,
                        AutoAction::Disable => AlarmEnable::Disarmed,
                    };
                    self.alarm.set_enable(&action);
                }
                AlarmJob::DailyAuto(_, AutoDevice::Lights, lights_action) => {
                    self.night_lights
                        .set_auto_active(lights_action == &AutoAction::Enable);
                }
            },
            _ => {}
        }

        Ok(Verdict::default())
    }

    // Unschedule daily jobs if auto mode is disabled, update time if changed
    fn update_job(&mut self, job: &mut Self::Job) -> UpdateJobVerdict {
        match job {
            AlarmJob::DailyAuto(time, AutoDevice::Alarm, _) => {
                if !self.config.auto_alarm_enable {
                    info!("Unscheduling daily auto alarm job");
                    return UpdateJobVerdict::Unschedule;
                } else if self.config.auto_alarm_enable_time != *time {
                    *time = self.config.auto_alarm_enable_time;
                }
            }
            AlarmJob::DailyAuto(time, AutoDevice::Lights, _) => {
                if !self.config.auto_lights_enable {
                    info!("Unscheduling daily auto lights job");
                    return UpdateJobVerdict::Unschedule;
                } else if self.config.auto_lights_enable_time != *time {
                    *time = self.config.auto_lights_enable_time;
                }
            }
        }

        UpdateJobVerdict::Keep
    }

    fn handle_action(
        &mut self,
        action: &Self::Action,
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<Self::Action>, DeviceError> {
        match action {
            Action::GetStatus => {}
            Action::GetConfig => {}
            Action::SetConfig(partial) => self.patch_config(partial.clone(), ctx)?,
            Action::SetAlarm(state) => {
                let set_alarm_result = self.alarm.set_enable(state);
                if set_alarm_result.is_falling() {
                    let mut command = OutdoorAlarmCommand::default();
                    command.set_siren(Xps::Reset);
                    return Ok(ActionVerdict::ActionPendingOn(command.into_request()));
                } else if set_alarm_result.is_rising() {
                    if self.ios.sabotage {
                        self.alarm.set_enable(&AlarmEnable::Disarmed);
                        return Ok(ActionVerdict::ActionRejected(
                            "Cannot arm alarm while sabotage detected".to_string(),
                        ));
                    }
                }
            }
            Action::SetLights(action) => {
                let mut command = OutdoorAlarmCommand::default();
                command.set_east_light((&action.east).into());
                command.set_south_light((&action.south).into());

                return Ok(ActionVerdict::ActionPendingOn(command.into_request()));
            }
            Action::SirenAction(action) => {
                let mut command = OutdoorAlarmCommand::default();
                match action {
                    SirenAction::ForceOff => {
                        command.set_siren(Xps::Reset);
                    }
                }

                return Ok(ActionVerdict::ActionPendingOn(command.into_request()));
            }
        }

        let status = self.get_state();
        Ok(ActionVerdict::ActionResult(status))
    }

    fn handle_frame(
        &mut self,
        _frame: &caniot::ResponseData,
        as_class_blc: &Option<crate::caniot::BoardClassTelemetry>,
        _ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        if let Some(caniot::BoardClassTelemetry::Class0(telemetry)) = as_class_blc {
            let new_state = DeviceIOState {
                siren: telemetry.rl1,
                detectors: [telemetry.in1, telemetry.in2],
                lights: [telemetry.oc1, telemetry.oc2],
                sabotage: telemetry.in4,
            };

            let now = Utc::now();

            return Ok(self
                .update_state(new_state, &now)
                .map(Verdict::Request)
                .unwrap_or_default());
        }
        Ok(Verdict::default())
    }

    fn handle_action_result(
        &self,
        _delayed_action: &Self::Action,
        _completed_by: Response,
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        Ok(self.get_state())
    }

    fn get_metrics(&self) -> Vec<String> {
        let alarm_label = SensorLabel::Controller("alarm".to_string());
        let outdoor_label = SensorLabel::Install("outdoor".to_string());
        let south_label = SensorLabel::Location("south".to_string());
        let east_label = SensorLabel::Location("east".to_string());

        let metrics = vec![
            // Counters
            format_metric(
                "detector_triggered_count",
                self.stats.south_detector_triggered_count,
                vec![&alarm_label, &outdoor_label, &south_label],
            ),
            format_metric(
                "detector_triggered_count",
                self.stats.east_detector_triggered_count,
                vec![&alarm_label, &outdoor_label, &east_label],
            ),
            format_metric(
                "sabotage_triggered_count",
                self.stats.sabotage_triggered_count,
                vec![&alarm_label, &outdoor_label],
            ),
            format_metric(
                "sirens_triggered_count",
                self.stats.sirens_triggered_count,
                vec![&alarm_label, &outdoor_label],
            ),
            // DeviceIOState metrics
            format_metric(
                "siren",
                self.ios.siren as u32,
                vec![&alarm_label, &outdoor_label],
            ),
            format_metric(
                "detector",
                self.ios.get_east_detector() as u32,
                vec![&alarm_label, &outdoor_label, &east_label],
            ),
            format_metric(
                "detector",
                self.ios.get_south_detector() as u32,
                vec![&alarm_label, &outdoor_label, &south_label],
            ),
            format_metric(
                "sabotage",
                self.ios.sabotage as u32,
                vec![&alarm_label, &outdoor_label],
            ),
            format_metric(
                "light",
                self.ios.get_south_light() as u32,
                vec![&alarm_label, &outdoor_label, &south_label],
            ),
            format_metric(
                "light",
                self.ios.get_east_light() as u32,
                vec![&alarm_label, &outdoor_label, &east_label],
            ),
            // Alarm state
            format_metric(
                "alarm_enabled",
                self.alarm.is_armed() as u32,
                vec![&alarm_label, &outdoor_label],
            ),
            // AlarmConfig metrics
            format_metric(
                "alarm_auto_enable",
                self.config.auto_alarm_enable as u32,
                vec![&alarm_label, &outdoor_label],
            ),
            format_metric(
                "lights_auto_enable",
                format!("{}", self.config.auto_lights_enable as u32),
                vec![&alarm_label, &outdoor_label],
            ),
        ];

        metrics
    }
}
