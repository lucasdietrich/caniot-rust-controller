use super::super::Behavior;
use crate::caniot::{self as ct, HeatingControllerCommand, HeatingControllerPayload, HeatingMode};

pub struct HeatersController {
    modes: [HeatingMode; 4],
    power_status: bool,
}

impl HeatersController {
    pub fn new() -> Self {
        Self {
            modes: [HeatingMode::Stop; 4],
            power_status: true,
        }
    }
}

impl Behavior for HeatersController {
    fn on_command(&mut self, endpoint: &ct::Endpoint, payload: Vec<u8>) -> Option<ct::ErrorCode> {
        if endpoint == &ct::Endpoint::ApplicationDefault {
            let command = HeatingControllerCommand::try_from(payload.as_slice()).unwrap();

            self.modes[0] = command.modes[0];
            self.modes[1] = command.modes[1];
            self.modes[2] = command.modes[2];
            self.modes[3] = command.modes[3];
        }
        None
    }

    fn on_telemetry(&mut self, endpoint: &ct::Endpoint) -> Option<Vec<u8>> {
        if endpoint == &ct::Endpoint::ApplicationDefault {
            let telemetry = HeatingControllerPayload {
                modes: self.modes,
                power_status: self.power_status,
            };

            return Some(telemetry.try_into().unwrap());
        }

        None
    }

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<ct::ErrorCode> {
        None
    }

    fn set_did(&mut self, _did: &ct::DeviceId) {
        // Do nothing
    }
}
