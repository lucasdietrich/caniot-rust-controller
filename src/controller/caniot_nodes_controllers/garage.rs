use crate::{
    caniot::Xps,
    controller::{
        ActionResultTrait, ActionTrait, ActionVerdict, DeviceAlert, DeviceControllerInfos,
        DeviceControllerTrait, DeviceError, Verdict,
    },
    ha::LOCATION_GARAGE,
    utils::{format_metric, monitorable_state::StateMonitor, SensorLabel},
};

use self::traits::ClassCommandTrait;

use super::super::super::caniot::*;

const CONTROLLER_NAME: &str = "garage";

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorCommand {
    // todo concept #[crl1(PulseOn)]
    pub left_door_activate: bool,
    // todo concept #[crl2(PulseOn)]
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

pub struct GarageIOState {
    pub left_door_open: bool,
    pub right_door_open: bool,
    pub gate_open: bool,
}

impl From<&class0::Telemetry> for GarageIOState {
    fn from(payload: &class0::Telemetry) -> Self {
        Self {
            left_door_open: payload.in3.into(),
            right_door_open: payload.in4.into(),
            gate_open: payload.in2,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GarageDoorStatus {
    pub left_door_status: StateMonitor<DoorState>,
    pub right_door_status: StateMonitor<DoorState>,
    pub gate_open: StateMonitor<bool>,
}

impl GarageDoorStatus {
    pub fn init(ios: GarageIOState) -> Self {
        Self {
            left_door_status: StateMonitor::init(ios.left_door_open.into()),
            right_door_status: StateMonitor::init(ios.right_door_open.into()),
            gate_open: StateMonitor::init(ios.gate_open),
        }
    }

    fn update(&mut self, ios: GarageIOState, stats: &mut GarageDoorStats) {
        self.left_door_status
            .update(ios.left_door_open.into())
            .map(|old| {
                if old.is_closed() {
                    stats.left_door_open_count += 1;
                }
            });

        self.right_door_status
            .update(ios.right_door_open.into())
            .map(|old| {
                if old.is_closed() {
                    stats.right_door_open_count += 1;
                }
            });

        self.gate_open.update(ios.gate_open).map(|old| {
            if !old {
                stats.gate_open_count += 1;
            }
        });
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct GarageDoorStats {
    pub left_door_open_count: u32,
    pub right_door_open_count: u32,
    pub gate_open_count: u32,
    pub left_door_command_sent: u32,
    pub right_door_command_sent: u32,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DoorState {
    #[default]
    Open,
    Closed,
}

impl DoorState {
    pub fn is_open(&self) -> bool {
        !matches!(self, DoorState::Closed)
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }
    pub fn progress(&self) -> Option<u8> {
        match self {
            DoorState::Open => None,
            DoorState::Closed => None,
        }
    }
}

impl Into<bool> for DoorState {
    fn into(self) -> bool {
        match self {
            DoorState::Open => true,
            DoorState::Closed => false,
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

#[derive(Debug, Default)]
pub struct GarageController {
    status: Option<GarageDoorStatus>,
    stats: GarageDoorStats,
}

impl DeviceControllerTrait for GarageController {
    type Action = GarageAction;
    type Job = ();
    type Config = ();

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new(
            CONTROLLER_NAME,
            Some(LOCATION_GARAGE),
            Some("Portes de garage"),
            Some("garage"),
        )
    }

    fn get_config(&self) -> &Self::Config {
        &()
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
                if command.left_door_activate {
                    self.stats.left_door_command_sent += 1;
                }

                if command.right_door_activate {
                    self.stats.right_door_command_sent += 1;
                }

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
                let ios = GarageIOState::from(telemetry);
                if let Some(ref mut status) = self.status {
                    status.update(ios, &mut self.stats);
                } else {
                    self.status = Some(GarageDoorStatus::init(ios));
                }
            }
        }

        Ok(Verdict::None)
    }

    fn get_alert(&self) -> Option<DeviceAlert> {
        self.status.as_ref().and_then(|s| {
            if s.left_door_status.is_open() || s.right_door_status.is_open() || *s.gate_open {
                Some(DeviceAlert::new_warning("Porte(s) de garage ouverte(s)"))
            } else {
                None
            }
        })
    }

    fn get_metrics(&self) -> Vec<String> {
        let label_ctrl = SensorLabel::Controller(CONTROLLER_NAME.to_string());
        let label_location = SensorLabel::Install("indoor".to_string());
        let label_left_door = SensorLabel::Location("left".to_string());
        let label_right_door = SensorLabel::Location("right".to_string());
        let label_gate = SensorLabel::Location("gate".to_string());

        let mut metrics = vec![];

        if let Some(ref status) = self.status {
            metrics.push(format_metric(
                "door_open",
                status.left_door_status.is_open() as u32,
                vec![&label_ctrl, &label_location, &label_left_door],
            ));

            metrics.push(format_metric(
                "door_open",
                status.right_door_status.is_open() as u32,
                vec![&label_ctrl, &label_location, &label_right_door],
            ));

            metrics.push(format_metric(
                "door_open",
                *status.gate_open as u32,
                vec![&label_ctrl, &label_location, &label_gate],
            ));

            metrics.push(format_metric(
                "door_open_count",
                self.stats.left_door_open_count,
                vec![&label_ctrl, &label_location, &label_left_door],
            ));

            metrics.push(format_metric(
                "door_open_count",
                self.stats.right_door_open_count,
                vec![&label_ctrl, &label_location, &label_right_door],
            ));

            metrics.push(format_metric(
                "door_open_count",
                self.stats.gate_open_count,
                vec![&label_ctrl, &label_location, &label_gate],
            ));

            metrics.push(format_metric(
                "door_command_sent",
                self.stats.left_door_command_sent,
                vec![&label_ctrl, &label_location, &label_left_door],
            ));

            metrics.push(format_metric(
                "door_command_sent",
                self.stats.right_door_command_sent,
                vec![&label_ctrl, &label_location, &label_right_door],
            ));
        }

        metrics
    }
}
