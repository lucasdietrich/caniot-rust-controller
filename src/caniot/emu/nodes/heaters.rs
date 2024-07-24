use super::super::Behavior;
use crate::{
    caniot::{self as ct, AsPayload, HeatingMode},
    controller::heaters::types::{HeatingControllerCommand, HeatingControllerTelemetry},
};

pub struct HeatersController {
    modes: [HeatingMode; 4],
    power_status: bool,
}

impl Default for HeatersController {
    fn default() -> Self {
        Self {
            modes: [
                HeatingMode::Stop,
                HeatingMode::Stop,
                HeatingMode::Stop,
                HeatingMode::Stop,
            ],
            power_status: true,
        }
    }
}

impl Behavior for HeatersController {
    fn on_command(
        &mut self,
        endpoint: &ct::Endpoint,
        payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<ct::ErrorCode> {
        if endpoint == &ct::Endpoint::ApplicationDefault {
            let command = HeatingControllerCommand::try_from_raw(&payload).unwrap();

            for (i, mode) in command.modes.iter().enumerate() {
                if mode != &HeatingMode::None {
                    self.modes[i] = *mode;
                }
            }
        }
        Some(ct::ErrorCode::Ok)
    }

    fn on_telemetry(&mut self, endpoint: &ct::Endpoint) -> Option<Vec<u8>> {
        if endpoint == &ct::Endpoint::ApplicationDefault {
            let telemetry = HeatingControllerTelemetry {
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
