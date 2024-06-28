use log::debug;

use crate::{
    caniot::{self, Response, Xps},
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
}

impl AlarmContext {
    pub fn set_enable(&mut self, state: &AlarmEnable) {
        self.state = state.clone();
    }

    pub fn is_armed(&self) -> bool {
        matches!(self.state, AlarmEnable::Armed)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeviceState {
    pub siren: bool,          // true if siren is on
    pub detectors: [bool; 2], // east, south (true if presence detected)
    pub lights: [bool; 2],    // east, south (true if lights are on)
    pub sabotage: bool,       // false if sabotage detected
}

#[derive(Debug, Clone, Default)]
pub struct AlarmController {
    pub device: DeviceState,

    pub alarm: AlarmContext,
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
    /// Returns `true` if the device state has changed and immediate processing is required,
    /// otherwise `false`.
    pub fn update_state(&mut self, new_state: &DeviceState) -> bool {
        self.device = new_state.clone();

        false
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

                if self.update_state(&new_state) {
                    ctx.request_process_immediate();
                }

                Ok(Verdict::default())
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
