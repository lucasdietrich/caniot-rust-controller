use std::str::FromStr;

use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, Utc};
use log::{info, warn};

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

    pub last_siren_activation: Option<DateTime<Local>>,
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
    //
    // Auto mode is automatically turns on the lights when presence is detected
    pub auto: bool,

    // Range of time where the lights are automatically turned on
    pub auto_range: [NaiveTime; 2], // lower bound, upper bound

    // Desired duration for the lights to stay on when presence is detected
    pub desired_duration: Duration,
}

impl Default for NightLightsContext {
    fn default() -> Self {
        Self {
            // Auto mode is enabled by default
            auto: true,
            auto_range: [
                NaiveTime::from_hms_opt(20, 0, 0).expect("Invalid auto lights start time"),
                NaiveTime::from_hms_opt(6, 0, 0).expect("Invalid auto lights stop time"),
            ],
            desired_duration: Duration::seconds(60),
        }
    }
}

impl NightLightsContext {
    #[allow(dead_code)]
    pub fn set_auto(&mut self, state: bool) {
        self.auto = state;
    }

    pub fn is_active(&self, now: &NaiveTime) -> bool {
        if !self.auto {
            return false;
        }

        let [lower_b, upper_b] = self.auto_range;
        if lower_b < upper_b {
            now >= &lower_b && now < &upper_b
        } else {
            now >= &lower_b || now < &upper_b
        }
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

#[derive(Debug, Clone, Default)]
pub struct AlarmController {
    pub ios: DeviceIOState,

    pub alarm: AlarmContext,
    pub night_lights: NightLightsContext,
}

#[derive(Debug, Clone)]
pub struct AlarmControllerState {
    pub ios: DeviceIOState,

    pub alarm_enabled: bool,
}

impl ActionResultTrait for AlarmControllerState {}

impl AlarmController {
    /// Updates the device state with a new state provided as input.
    ///
    /// Arguments:
    ///
    /// * `new_state`: New state to update the device with.
    ///
    /// Returns:
    ///
    /// Returns `request_data` if a request should be sent to the device.
    pub fn update_state(&mut self, new_state: DeviceIOState) -> Option<RequestData> {
        self.ios = new_state;

        let mut command = OutdoorAlarmCommand::default();

        let (south_detector_result, east_detector_result, sabotage_result) = (
            self.alarm.south_detector.update(self.ios.detectors[0]),
            self.alarm.east_detector.update(self.ios.detectors[1]),
            self.alarm.sabotage.update(self.ios.sabotage),
        );

        let now = Local::now();
        let mut trigger_siren = false;

        let detector_triggered =
            south_detector_result.is_rising() || east_detector_result.is_rising();
        let sabotage_triggered = sabotage_result.is_rising();

        if detector_triggered {
            info!("Presence detected");
            if self.night_lights.is_active(&now.time()) {
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
            command.set_siren(Xps::PulseOn);
            command.set_east_light(Xps::PulseOn);
            command.set_south_light(Xps::PulseOn);
            self.alarm.last_siren_activation = Some(now);
            self.alarm.siren_triggered_count += 1;
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
    AlarmAutoEnable,
    AlarmAutoDisable,
    AutoLightsAutoEnable,
    AutoLightsAutoDisable,
}

impl DevCtrlSchedJobTrait for AlarmJob {
    fn get_scheduling(&self) -> Scheduling {
        match self {
            // AlarmJob::Test => Scheduling::OnceAt(
            //     NaiveDateTime::from_str("2024-07-12T22:05:00.884862963").unwrap(),
            // ),
            AlarmJob::AlarmAutoEnable => {
                Scheduling::Daily(NaiveTime::from_hms_opt(14, 36, 0).unwrap())
            }
            AlarmJob::AlarmAutoDisable => {
                Scheduling::Daily(NaiveTime::from_hms_opt(14, 37, 0).unwrap())
            }
            AlarmJob::AutoLightsAutoEnable => {
                Scheduling::Daily(NaiveTime::from_hms_opt(20, 0, 0).unwrap())
            }
            AlarmJob::AutoLightsAutoDisable => {
                Scheduling::Daily(NaiveTime::from_hms_opt(6, 0, 0).unwrap())
            }
        }
    }
}

impl DeviceControllerTrait for AlarmController {
    type Action = Action;
    type SchedJob = AlarmJob;

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
        job_timestamp: DateTime<Utc>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        println!("Processing outdoor alarm: {:?}", job);

        match job {
            // Declare jobs that should be executed when the device is added
            DeviceJobImpl::DeviceAdd => {
                ctx.add_job(AlarmJob::AlarmAutoEnable);
                ctx.add_job(AlarmJob::AlarmAutoDisable);
                ctx.add_job(AlarmJob::AutoLightsAutoEnable);
                ctx.add_job(AlarmJob::AutoLightsAutoDisable);
            }
            DeviceJobImpl::Scheduled(job) => match job {
                AlarmJob::AlarmAutoEnable => {
                    self.alarm.set_enable(&AlarmEnable::Armed);
                }
                AlarmJob::AlarmAutoDisable => {
                    self.alarm.set_enable(&AlarmEnable::Disarmed);
                }
                AlarmJob::AutoLightsAutoEnable => {
                    // activate auto lights
                }
                AlarmJob::AutoLightsAutoDisable => {
                    // deactivate auto lights
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
        _ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        if let Some(caniot::BoardClassTelemetry::Class0(telemetry)) = as_class_blc {
            let new_state = DeviceIOState {
                siren: telemetry.rl1,
                detectors: [telemetry.in1, telemetry.in2],
                lights: [telemetry.oc1, telemetry.oc2],
                sabotage: telemetry.in4,
            };

            return Ok(self
                .update_state(new_state)
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
