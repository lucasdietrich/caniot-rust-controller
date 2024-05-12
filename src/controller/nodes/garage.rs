use crate::caniot::Xps;

use super::super::super::caniot::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorCommand {
    // #[crl1(PulseOn)]
    pub left_door_activate: bool,
    // #[crl2(PulseOn)]
    pub right_door_activate: bool,
}

#[allow(clippy::all)]
impl Into<class0::Command> for GarageDoorCommand {
    fn into(self) -> class0::Command {
        class0::Command {
            crl1: if self.left_door_activate {
                Xps::PulseOn
            } else {
                Xps::None
            },
            crl2: if self.right_door_activate {
                Xps::PulseOn
            } else {
                Xps::None
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorStatus {
    pub left_door_status: bool,
    pub right_door_status: bool,
    pub garage_light_status: bool,
}

impl From<class0::Telemetry> for GarageDoorStatus {
    fn from(payload: class0::Telemetry) -> Self {
        Self {
            left_door_status: payload.in3,
            right_door_status: payload.in4,
            garage_light_status: payload.in2,
        }
    }
}

#[derive(Debug, Default)]
pub struct GarageController {
    status: GarageDoorStatus,
}

impl GarageController {}
