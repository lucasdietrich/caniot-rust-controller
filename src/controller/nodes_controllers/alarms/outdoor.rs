use chrono::{DateTime, Duration, NaiveTime, Timelike, Utc};
use log::{debug, info, warn};

use crate::{
    caniot::{self, traits::ClassCommandTrait, RequestData, Response, Xps},
    controller::{
        alarms::{actions::SirenAction, types::OutdoorAlarmCommand},
        alert::DeviceAlert,
        ActionResultTrait, ActionTrait, ActionVerdict, DeviceControllerInfos,
        DeviceControllerTrait, DeviceError, ProcessContext, Verdict,
    },
};

use super::actions::{Action, AlarmEnable};

#[derive(Debug, Clone, Default)]
pub struct AlarmContext {
    pub state: AlarmEnable,

    pub last_siren_activation: Option<DateTime<Utc>>,
    pub siren_triggered_count: u32,
}

impl AlarmContext {
    pub fn set_enable(&mut self, state: &AlarmEnable) {
        self.state = state.clone();
    }

    pub fn is_armed(&self) -> bool {
        matches!(self.state, AlarmEnable::Armed)
    }
}

#[derive(Debug, Clone)]
pub struct NightLightsContext {
    // Auto mode is enabled
    //
    // Auto mode is automatically turns on the lights when presence is detected
    pub auto: bool,

    // Range of time where the lights are automatically turned on
    pub auto_range: [NaiveTime; 2], // Start, stop

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
    pub fn set_auto(&mut self, state: bool) {
        self.auto = state;
    }

    pub fn is_active(&self, now: &NaiveTime) -> bool {
        if !self.auto {
            return false;
        }

        let start = self.auto_range[0];
        let stop = self.auto_range[1];

        if start < stop {
            now >= &start && now < &stop
        } else {
            now >= &start || now < &stop
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeviceState {
    pub siren: bool,          // true if siren is on
    pub detectors: [bool; 2], // east, south (true if presence detected)
    pub lights: [bool; 2],    // east, south (true if lights are on)
    pub sabotage: bool,       // false if sabotage detected
}

impl DeviceState {
    pub fn is_siren_on(&self) -> bool {
        self.siren
    }

    pub fn is_presence_detected(&self) -> bool {
        self.detectors.iter().any(|&d| d)
    }

    pub fn is_sabotage_detected(&self) -> bool {
        self.sabotage
    }
}

#[derive(Debug, Clone, Default)]
pub struct AlarmController {
    pub device: DeviceState,

    pub alarm: AlarmContext,
    pub night_lights: NightLightsContext,
}

#[derive(Debug, Clone)]
pub struct AlarmControllerState {
    pub device: DeviceState,

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
    pub fn update_state(&mut self, new_state: &DeviceState) -> Option<RequestData> {
        self.device = new_state.clone();
        let mut command = OutdoorAlarmCommand::default();

        if self.device.is_presence_detected() {
            info!("Presence detected");

            let now = Utc::now();

            if self.night_lights.is_active(&now.time()) {
                info!("Lights turned on");
                command.set_east_light(Xps::PulseOn);
                command.set_south_light(Xps::PulseOn);
            }

            if self.alarm.is_armed() {
                warn!("Presence detected while alarm is armed, activating siren");
                command.set_siren(Xps::PulseOn);
                command.set_east_light(Xps::PulseOn);
                command.set_south_light(Xps::PulseOn);
                self.alarm.last_siren_activation = Some(now);
                self.alarm.siren_triggered_count += 1;
            }
        } else if self.device.is_sabotage_detected() {
            warn!("Sabotage detected on the outdoor alarm");
            let mut command = OutdoorAlarmCommand::default();
            command.set_siren(Xps::PulseOn);
        }

        // If command has an actual effect, send it to the device
        if command.has_effect() {
            Some(command.into_request())
        } else {
            None
        }
    }

    /// Returns the current state of the device.
    pub fn get_state(&self) -> AlarmControllerState {
        AlarmControllerState {
            device: self.device.clone(),
            alarm_enabled: self.alarm.is_armed(),
        }
    }
}

impl DeviceControllerTrait for AlarmController {
    type Action = Action;

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new(
            "outdoor_alarm",
            Some("Controleur d'alarme extérieure"),
            Some("alarms"),
        )
    }

    fn get_alert(&self) -> Option<DeviceAlert> {
        if self.alarm.is_armed() {
            Some(DeviceAlert::new_ok("Alarme extérieure activée"))
        } else {
            None
        }
    }

    fn handle_action(
        &mut self,
        action: &Self::Action,
        _ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<Self::Action>, DeviceError> {
        debug!("Handling action: {:?}", action);

        match action {
            Action::GetStatus => {}
            Action::SetAlarm(state) => {
                self.alarm.set_enable(state);
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
    ) -> Result<crate::controller::Verdict, DeviceError> {
        match as_class_blc {
            Some(caniot::BoardClassTelemetry::Class0(telemetry)) => {
                let new_state = DeviceState {
                    siren: telemetry.rl1,
                    detectors: [telemetry.in1, telemetry.in2],
                    lights: [telemetry.oc1, telemetry.oc2],
                    sabotage: telemetry.in4,
                };

                if let Some(req) = self.update_state(&new_state) {
                    Ok(Verdict::Request(req))
                } else {
                    Ok(Verdict::default())
                }
            }
            _ => Ok(Verdict::default()),
        }
    }

    fn handle_action_result(
        &self,
        _delayed_action: &Self::Action,
        _completed_by: Response,
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        Ok(self.get_state())
    }
}
