use log::debug;

use crate::{
    caniot::{self, class0, traits::ClassCommandTrait, Xps},
    controller::{
        ActionResultTrait, ActionTrait, ActionVerdict, DeviceControllerInfos,
        DeviceControllerTrait, DeviceError, ProcessContext, Verdict,
    },
};

#[derive(Debug, Clone, Default)]
pub enum AlarmEnable {
    Disarmed,
    #[default]
    Armed,
}

#[derive(Debug, Clone, Default)]
pub enum LightAction {
    #[default]
    None,
    On,
    Off,
    Toggle,
}

impl Into<Xps> for &LightAction {
    fn into(self) -> Xps {
        match self {
            LightAction::None => Xps::None,
            LightAction::On => Xps::SetOn,
            LightAction::Off => Xps::SetOff,
            LightAction::Toggle => Xps::Toggle,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LightsActions {
    pub south: LightAction,
    pub east: LightAction,
}

impl LightsActions {
    pub fn new(south: Option<LightAction>, east: Option<LightAction>) -> Self {
        Self {
            south: south.unwrap_or_default(),
            east: east.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SirenAction {
    ForceOff,
}

#[derive(Debug)]
pub enum Action {
    GetStatus,
    SetAlarm(AlarmEnable),
    SetLights(LightsActions),
    SirenAction(SirenAction),
}

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

#[derive(Default)]
pub struct OutdoorAlarmCommand(pub class0::Command);

impl OutdoorAlarmCommand {
    pub fn new(south: Xps, east: Xps, siren: Xps) -> Self {
        OutdoorAlarmCommand(class0::Command {
            coc1: south,
            coc2: east,
            crl1: siren,
            crl2: Xps::None,
        })
    }

    pub fn set_siren(&mut self, cmd: Xps) {
        self.0.crl1 = cmd;
    }

    pub fn set_east_light(&mut self, cmd: Xps) {
        self.0.coc1 = cmd;
    }

    pub fn set_south_light(&mut self, cmd: Xps) {
        self.0.coc2 = cmd;
    }

    pub fn into_request(self) -> caniot::RequestData {
        self.0.to_request()
    }
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

impl ActionTrait for Action {
    type Result = AlarmControllerState;
}

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
        DeviceControllerInfos::new("Outdoor Alarm Controller")
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
        as_class_blc: &Option<crate::caniot::BlcClassTelemetry>,
        ctx: &mut ProcessContext,
    ) -> Result<crate::controller::Verdict, DeviceError> {
        match as_class_blc {
            Some(caniot::BlcClassTelemetry::Class0(telemetry)) => {
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
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        Ok(self.get_state())
    }
}
