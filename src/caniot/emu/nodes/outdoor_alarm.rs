use std::{thread, time::Duration};

use super::super::Behavior;
use crate::{
    caniot::{self as ct, class0, emu::helpers::EmuXps, Temperature},
    utils::expirable::ExpirableTrait,
};

#[derive(Default)]
pub struct OutdoorAlarmController {
    lights: [EmuXps; 2],         // oc1, oc2
    siren: EmuXps,               // rl1
    presence_sensors: [bool; 2], // in1, in2
    sabotage: bool,              // in4
}

impl Behavior for OutdoorAlarmController {
    fn on_command(&mut self, endpoint: &ct::Endpoint, payload: Vec<u8>) -> Option<ct::ErrorCode> {
        if endpoint == &ct::Endpoint::BoardControl {
            if let Ok(command) = class0::Command::try_from(payload.as_slice()) {
                self.lights[0].apply(&command.coc1);
                self.lights[1].apply(&command.coc2);
                self.siren.apply(&command.crl1);

                Some(ct::ErrorCode::Ok)
            } else {
                Some(ct::ErrorCode::Eframe)
            }
        } else {
            None
        }
    }

    fn on_telemetry(&mut self, endpoint: &ct::Endpoint) -> Option<Vec<u8>> {
        // thread::sleep(Duration::from_millis(500));

        if endpoint == &ct::Endpoint::BoardControl {
            let mut telemetry = class0::Telemetry::default();

            telemetry.in1 = self.presence_sensors[0];
            telemetry.in2 = self.presence_sensors[1];
            telemetry.in4 = self.sabotage;
            telemetry.oc1 = self.lights[0].get_state();
            telemetry.poc1 = self.lights[0].pulse_pending();
            telemetry.oc2 = self.lights[1].get_state();
            telemetry.poc2 = self.lights[1].pulse_pending();
            telemetry.rl1 = self.siren.get_state();
            telemetry.prl1 = self.siren.pulse_pending();
            telemetry.temp_in = Temperature::random();
            telemetry.temp_out = [
                Temperature::random(),
                Temperature::INVALID,
                Temperature::INVALID,
            ];

            Some(telemetry.into())
        } else {
            None
        }
    }

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<ct::ErrorCode> {
        None
    }

    fn get_remaining_to_event_ms(&self) -> Option<u64> {
        [&self.lights[0], &self.lights[1], &self.siren]
            .iter()
            .ttl()
            .map(|duration| duration.as_millis() as u64)
    }

    fn process(&mut self) -> Option<ct::Endpoint> {
        if self.lights[0].pulse_process().is_some()
            || self.lights[1].pulse_process().is_some()
            || self.siren.pulse_process().is_some()
        {
            Some(ct::Endpoint::BoardControl)
        } else {
            None
        }
    }

    fn set_did(&mut self, _did: &ct::DeviceId) {
        // Do nothing
    }
}
