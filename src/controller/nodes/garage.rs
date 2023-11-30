use crate::{
    caniot::{self, CaniotError, DeviceId, Frame, Response},
    controller::{ControllerError, ControllerHandle, DeviceHandle, traits::ControllerAPI, DeviceTrait, ManagedDeviceTrait, ManagedDeviceError},
};

use super::super::super::caniot::types::*;

pub const DEVICE_ID: DeviceId = DeviceId {
    class: 0,
    sub_id: 1,
};

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

#[derive(Debug)]
pub struct GarageController {
    device_id: caniot::DeviceId,
}

impl ManagedDeviceTrait for GarageController {
    // type Error = ManagedDeviceError;

    fn handle_frame(&mut self, frame: &caniot::Response) -> Result<(), ManagedDeviceError> {
        Err(ManagedDeviceError::UnsupportedFrame)
    }
}

impl DeviceTrait for GarageController {
    fn new(device_id: caniot::DeviceId) -> Self {
        Self { device_id }
    }

    fn get_did(&self) -> caniot::DeviceId {
        self.device_id
    }

    fn is_managed(&self) -> bool {
        true
    }
}

impl DeviceHandle<'_, GarageController> {
    pub async fn open_door(
        &mut self,
        activate_left: bool,
        activate_right: bool,
    ) -> Result<caniot::Response, ControllerError> {

        let command = GarageDoorCommand {
            left_door_activate: activate_left,
            right_door_activate: activate_right,
        };
        let command = BlcCommand {
            class_payload: BlcClassCommand::Class0(command.into()),
            sys: SystemCommand::default(),
        };
        let payload: [u8; 8] = command.into();
        let request = caniot::RequestData::Command {
            endpoint: caniot::Endpoint::BoardControl,
            payload: payload.into(),
        };

        self.controller_handle.query(
            caniot::Request { device_id: self.device.get_did(), data: request }, 1000
        ).await
    }
}

#[derive(Debug, Clone)]
pub struct GarageHandle<'a> {
    device_id: caniot::DeviceId,
    controller_handler: &'a ControllerHandle,
}

impl GarageHandle<'_> {
    pub fn new(controller_handler: &ControllerHandle) -> GarageHandle {
        GarageHandle {
            device_id: DEVICE_ID, // TODO
            controller_handler,
        }
    }

    pub async fn send_command(
        &self,
        activate_left: bool,
        activate_right: bool,
    ) -> Result<Response, ControllerError> {
        let command = GarageDoorCommand {
            left_door_activate: activate_left,
            right_door_activate: activate_right,
        };
        let command = BlcCommand {
            class_payload: BlcClassCommand::Class0(command.into()),
            sys: SystemCommand::default(),
        };
        let payload: [u8; 8] = command.into();
        let request = caniot::RequestData::Command {
            endpoint: caniot::Endpoint::BoardControl,
            payload: payload.into(),
        };
        let frame = Frame {
            device_id: self.device_id,
            data: request,
        };

        // self.controller_handler
        //     .query(frame, 1000)
        //     .await

        Err(ControllerError::UnsupportedQuery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enc() {
        let cmd = GarageDoorCommand {
            left_door_activate: true,
            right_door_activate: true,
        };
        let cmd: Class0Command = cmd.into();
        assert_eq!(cmd.crl1, Xps::PulseOn);
        assert_eq!(cmd.crl2, Xps::PulseOn);
        assert_eq!(cmd.coc1, Xps::None);
        assert_eq!(cmd.coc2, Xps::None);
    }

    #[test]
    fn dec() {
        let payload = Class0Payload {
            in2: true,
            in3: true,
            in4: true,
            ..Default::default()
        };
        let status = GarageDoorStatus::from(payload);
        assert_eq!(status.left_door_status, true);
        assert_eq!(status.right_door_status, true);
        assert_eq!(status.garage_light_status, true);
    }
}
