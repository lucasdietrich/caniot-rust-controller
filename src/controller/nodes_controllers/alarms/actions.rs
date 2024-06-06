use crate::{caniot::Xps, controller::ActionTrait};

use super::outdoor::AlarmControllerState;

#[derive(Debug, Clone, Default)]
pub enum AlarmEnable {
    #[default]
    Disarmed,
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

impl ActionTrait for Action {
    type Result = AlarmControllerState;
}
