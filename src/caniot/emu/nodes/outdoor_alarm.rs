use std::{
    thread,
    time::{Duration, Instant},
};

use super::super::Behavior;
use crate::{
    caniot::{
        self as ct,
        class0::{self, Class0},
        emu::helpers::EmuXps,
        AsPayload, BoardClassCommand, Temperature,
    },
    grpcserver::EmuRequest,
    utils::expirable::ExpirableTrait,
};

const LIGHTS_PULSE_DURATION: Duration = Duration::from_secs(30);
const SIREN_PULSE_DURATION: Duration = Duration::from_secs(20);

#[derive(Default)]
pub struct OutdoorAlarmController {
    lights: [EmuXps; 2],         // oc1, oc2
    siren: EmuXps,               // rl1
    presence_sensors: [bool; 2], // in1, in2
    sabotage: bool,              // in4
}

impl OutdoorAlarmController {
    pub fn new() -> Self {
        Self {
            lights: [
                EmuXps::new(false, false, Some(LIGHTS_PULSE_DURATION)),
                EmuXps::new(false, false, Some(LIGHTS_PULSE_DURATION)),
            ],
            siren: EmuXps::new(false, false, Some(SIREN_PULSE_DURATION)),
            presence_sensors: [false, false],
            sabotage: false,
        }
    }
}

impl Behavior for OutdoorAlarmController {
    fn on_command(
        &mut self,
        endpoint: &ct::Endpoint,
        payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<ct::ErrorCode> {
        if endpoint == &ct::Endpoint::BoardControl {
            if let Ok(blc_cmd) = BoardClassCommand::<Class0>::try_from_raw(&payload) {
                let command = blc_cmd.class_payload;

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
        thread::sleep(Duration::from_millis(500));

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
            telemetry.temp_in = Temperature::random_full_range();
            telemetry.temp_out = [
                Temperature::random_full_range(),
                Temperature::INVALID,
                Temperature::INVALID,
            ];

            // Reset detector after sending telemetry as it is a one-shot event
            self.presence_sensors[0] = false;
            self.presence_sensors[1] = false;

            Some(telemetry.to_raw_vec())
        } else {
            None
        }
    }

    fn get_remaining_to_event(&self, now: &Instant) -> Option<Duration> {
        [&self.lights[0], &self.lights[1], &self.siren]
            .iter()
            .ttl(now)
    }

    fn process(&mut self, now: &Instant) -> Option<ct::Endpoint> {
        // TODO improve this
        self.lights[0].pulse_process(now);
        self.lights[1].pulse_process(now);
        self.siren.pulse_process(now);

        Some(ct::Endpoint::BoardControl)
    }

    fn set_did(&mut self, _did: &ct::DeviceId) {
        // Do nothing
    }

    fn on_emu_request(&mut self, event: EmuRequest) -> bool {
        match event {
            EmuRequest::OutdoorAlarmClear => {
                self.presence_sensors[0] = false;
                self.presence_sensors[1] = false;
                self.sabotage = false;
            }
            EmuRequest::OutdoorAlarmPresence => {
                self.presence_sensors[0] = true;
                self.presence_sensors[1] = true;
            }
            EmuRequest::OutdoorAlarmSabotage => {
                self.sabotage = true;
            }
            #[allow(unreachable_patterns)]
            _ => return false,
        }

        true
    }
}
