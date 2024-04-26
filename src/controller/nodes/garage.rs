use crate::{
    caniot::{self, Endpoint, Response, Xps},
    controller::{ControllerAPI, ControllerError},
};

use super::super::super::caniot::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorCommand {
    // #[crl1(PulseOn)]
    pub left_door_activate: bool,
    // #[crl2(PulseOn)]
    pub right_door_activate: bool,
}

#[allow(clippy::all)]
impl Into<Class0Command> for GarageDoorCommand {
    fn into(self) -> Class0Command {
        Class0Command {
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

impl From<Class0Payload> for GarageDoorStatus {
    fn from(payload: Class0Payload) -> Self {
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

impl GarageController {
    pub async fn handle_action(
        &mut self,
        api: &mut dyn ControllerAPI,
        command: GarageDoorCommand,
    ) -> Result<Option<Response>, ControllerError> {
        let command: Class0Command = command.into();
        let payload: [u8; 7] = command.into();

        // let resp = api
        //     .query_command(
        //         self.device_id,
        //         caniot::Endpoint::BoardControl,
        //         payload.into(),
        //         1000,
        //     )
        //     .await?;
        // Ok(Some(resp))
        todo!();
    }
}
