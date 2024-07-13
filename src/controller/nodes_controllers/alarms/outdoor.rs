use chrono::{DateTime, Duration, Local, NaiveTime, Utc};
use log::{info, warn};
use rocket::time::Date;
use serde::{Deserialize, Serialize};

use super::actions::{Action, AlarmEnable};
use crate::{
    caniot::{self, RequestData, Response, Xps},
    controller::{
        alarms::{actions::SirenAction, types::OutdoorAlarmCommand},
        alert::DeviceAlert,
        ActionResultTrait, ActionTrait, ActionVerdict, DevCtrlSchedJobTrait, DeviceControllerInfos,
        DeviceControllerTrait, DeviceError, DeviceJobImpl, ProcessContext, Verdict,
    },
    utils::{
        monitorable::{MonitorableResultTrait, ValueMonitor},
        Scheduling,
    },
};

#[derive(Debug, Clone, Default)]
pub struct AlarmContext {
    pub state: ValueMonitor<AlarmEnable>,

    pub last_siren_activation: Option<DateTime<Utc>>,
    pub siren_triggered_count: u32,

    pub south_detector: ValueMonitor<bool>,
    pub east_detector: ValueMonitor<bool>,
    pub sabotage: ValueMonitor<bool>,
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
    pub lights: [bool; 2],    // east, south (true if lights are on)
    pub sabotage: bool,       // false if sabotage detected
}

impl DeviceIOState {
    pub fn is_siren_on(&self) -> bool {
        self.siren
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct AlarmConfig {
    pub auto_alarm_enable: bool,
    pub auto_alarm_enable_time: NaiveTime,
    pub auto_alarm_disable_time: NaiveTime,
    pub alarm_siren_minimum_interval_seconds: u32,

    pub auto_lights_enable: bool,
    pub auto_lights_enable_time: NaiveTime,
    pub auto_lights_disable_time: NaiveTime,
}

#[derive(Debug, Clone, Default)]
pub struct AlarmController {
    pub ios: DeviceIOState,

    pub alarm: AlarmContext,
    pub night_lights: NightLightsContext,

    pub config: AlarmConfig,
}

#[derive(Debug, Clone)]
pub struct AlarmControllerState {
    pub ios: DeviceIOState,

    pub alarm_enabled: bool,
}

impl ActionResultTrait for AlarmControllerState {}

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
            self.alarm.south_detector.update(self.ios.detectors[0]),
            self.alarm.east_detector.update(self.ios.detectors[1]),
            self.alarm.sabotage.update(self.ios.sabotage),
        );

        let mut trigger_siren = false;

        let detector_triggered =
            south_detector_result.is_rising() || east_detector_result.is_rising();
        let sabotage_triggered = sabotage_result.is_rising();

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

        if sabotage_triggered {
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
                self.alarm.last_siren_activation = Some(*now);
                self.alarm.siren_triggered_count += 1;
            } else {
                warn!("Siren activation blocked by minimum interval");
            }
        }

        command.has_effect().then(|| command.into_request())
    }

    /// Returns the current state of the device.
    pub fn get_state(&self) -> AlarmControllerState {
        AlarmControllerState {
            ios: self.ios.clone(),
            alarm_enabled: self.alarm.is_armed(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlarmJob {
    AlarmAutoEnableDaily(NaiveTime),
    AlarmAutoDisableDaily(NaiveTime),
    AutoLightsAutoEnableDaily(NaiveTime),
    AutoLightsAutoDisableDaily(NaiveTime),
}

impl DevCtrlSchedJobTrait for AlarmJob {
    fn get_scheduling(&self) -> Scheduling {
        match self {
            AlarmJob::AlarmAutoEnableDaily(time) => Scheduling::Daily(*time),
            AlarmJob::AlarmAutoDisableDaily(time) => Scheduling::Daily(*time),
            AlarmJob::AutoLightsAutoEnableDaily(time) => Scheduling::Daily(*time),
            AlarmJob::AutoLightsAutoDisableDaily(time) => Scheduling::Daily(*time),
        }
    }
}

impl DeviceControllerTrait for AlarmController {
    type Action = Action;
    type SchedJob = AlarmJob;
    type Config = AlarmConfig;

    fn new(config: Option<&Self::Config>) -> Self {
        Self {
            config: config.cloned().unwrap_or_default(),
            ..Default::default()
        }
    }

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new(
            "outdoor_alarm",
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
        job: &DeviceJobImpl<Self::SchedJob>,
        _job_timestamp: DateTime<Utc>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        println!("Processing outdoor alarm: {:?}", job);

        match job {
            // Declare jobs that should be executed when the device is added
            DeviceJobImpl::DeviceAdd => {
                if self.config.auto_alarm_enable {
                    ctx.add_job(AlarmJob::AlarmAutoEnableDaily(
                        self.config.auto_alarm_enable_time,
                    ));
                    ctx.add_job(AlarmJob::AlarmAutoDisableDaily(
                        self.config.auto_alarm_disable_time,
                    ));
                }

                if self.config.auto_lights_enable {
                    ctx.add_job(AlarmJob::AutoLightsAutoEnableDaily(
                        self.config.auto_lights_enable_time,
                    ));
                    ctx.add_job(AlarmJob::AutoLightsAutoDisableDaily(
                        self.config.auto_lights_disable_time,
                    ));
                }
            }
            DeviceJobImpl::Scheduled(job) => match job {
                AlarmJob::AlarmAutoEnableDaily(_) => {
                    self.alarm.set_enable(&AlarmEnable::Armed);
                }
                AlarmJob::AlarmAutoDisableDaily(_) => {
                    self.alarm.set_enable(&AlarmEnable::Disarmed);
                }
                AlarmJob::AutoLightsAutoEnableDaily(_) => {
                    self.night_lights.set_auto_active(true);
                }
                AlarmJob::AutoLightsAutoDisableDaily(_) => {
                    self.night_lights.set_auto_active(false);
                }
            },
            _ => {}
        }

        Ok(Verdict::default())
    }

    fn handle_action(
        &mut self,
        action: &Self::Action,
        _ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<Self::Action>, DeviceError> {
        match action {
            Action::GetStatus => {}
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
        ctx: &mut ProcessContext,
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
}
