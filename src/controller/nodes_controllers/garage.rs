use chrono::{DateTime, Utc};

use crate::{
    caniot::Xps,
    controller::{
        ActionResultTrait, ActionTrait, ActionVerdict, DeviceControllerInfos,
        DeviceControllerTrait, DeviceError, Verdict,
    },
};

use self::traits::ClassCommandTrait;

use super::super::super::caniot::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorCommand {
    // #[crl1(PulseOn)]
    pub left_door_activate: bool,
    // #[crl2(PulseOn)]
    pub right_door_activate: bool,
}

impl GarageDoorCommand {
    pub const OPEN_LEFT: GarageDoorCommand = GarageDoorCommand {
        left_door_activate: true,
        right_door_activate: false,
    };

    pub const OPEN_RIGHT: GarageDoorCommand = GarageDoorCommand {
        left_door_activate: false,
        right_door_activate: true,
    };

    pub const OPEN_BOTH: GarageDoorCommand = GarageDoorCommand {
        left_door_activate: true,
        right_door_activate: true,
    };
}

#[allow(clippy::all)]
impl Into<class0::Command> for &GarageDoorCommand {
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
    pub left_door_status: DoorState,
    pub right_door_status: DoorState,
    pub gate_open: bool,
}

impl From<&class0::Telemetry> for GarageDoorStatus {
    fn from(payload: &class0::Telemetry) -> Self {
        Self {
            left_door_status: payload.in3.into(),
            right_door_status: payload.in4.into(),
            gate_open: payload.in2,
        }
    }
}

#[derive(Debug)]
pub enum GarageAction {
    GetStatus,
    SetStatus(GarageDoorCommand),
}

impl ActionTrait for GarageAction {
    type Result = Option<GarageDoorStatus>;
}

impl ActionResultTrait for Option<GarageDoorStatus> {}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DoorState {
    #[default]
    Open,
    Closed,
    Moving(u8),
}

impl DoorState {
    pub fn is_open(&self) -> bool {
        !matches!(self, DoorState::Closed)
    }

    pub fn progress(&self) -> Option<u8> {
        match self {
            DoorState::Open => None,
            DoorState::Moving(progress) => Some(*progress),
            DoorState::Closed => None,
        }
    }
}

impl Into<bool> for DoorState {
    fn into(self) -> bool {
        match self {
            DoorState::Open => true,
            DoorState::Closed => false,
            DoorState::Moving(..) => true,
        }
    }
}

impl From<bool> for DoorState {
    fn from(value: bool) -> Self {
        if value {
            DoorState::Open
        } else {
            DoorState::Closed
        }
    }
}

#[derive(Debug)]
struct RequestedState {
    state: DoorState,
    date: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct GarageController {
    left_door_triggered: Option<RequestedState>,
    right_door_triggered: Option<RequestedState>,
    status: Option<GarageDoorStatus>,
}

impl DeviceControllerTrait for GarageController {
    type Action = GarageAction;

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new("Garage Controller")
    }

    fn handle_action(
        &mut self,
        action: &Self::Action,
        _ctx: &mut crate::controller::ProcessContext,
    ) -> Result<crate::controller::ActionVerdict<Self::Action>, crate::controller::DeviceError>
    {
        match action {
            GarageAction::GetStatus => Ok(ActionVerdict::ActionResult(self.status.clone())),
            GarageAction::SetStatus(command) => {
                let blc0_command: class0::Command = command.into();
                Ok(ActionVerdict::ActionPendingOn(blc0_command.into_request()))
            }
        }
    }

    fn handle_action_result(
        &self,
        _delayed_action: &Self::Action,
        _completed_by: Response,
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        Ok(self.status.clone())
    }

    fn handle_frame(
        &mut self,
        _frame: &crate::caniot::ResponseData,
        as_class_blc: &Option<BoardClassTelemetry>,
        _ctx: &mut crate::controller::ProcessContext,
    ) -> Result<crate::controller::Verdict, crate::controller::DeviceError> {
        if let Some(telemetry) = as_class_blc {
            if let Some(telemetry) = telemetry.as_class0() {
                self.status = Some(telemetry.into());
            }
        }

        Ok(Verdict::None)
    }
}
