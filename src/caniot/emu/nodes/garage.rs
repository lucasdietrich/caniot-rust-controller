use std::time::Instant;

use log::debug;

use super::super::Behavior;
use crate::{
    caniot::{self as ct, class0, AsPayload, Temperature, Xps},
    utils::expirable::ExpirableTrait,
};

#[derive(Default, Debug)]
enum Door {
    Open,
    #[default]
    Closed,
    Opening(Option<Instant>), // Some() is the time the door started opening, None if the door stopped opening
    Closing(Option<Instant>), // Some() is the time the door started closing, None if the door stopped closing
}

impl Door {
    pub const OPENNING_DURATION_MS: u128 = 10_000;
    pub const CLOSING_DURATION_MS: u128 = 10_000;

    fn pulse_relay(&mut self) {
        debug!("Pulsing relay {:?}", self);

        *self = match self {
            Door::Open => Door::Closing(Some(Instant::now())),
            Door::Closed => Door::Opening(Some(Instant::now())),
            Door::Opening(None) => Door::Closing(Some(Instant::now())),
            Door::Closing(None) => Door::Opening(Some(Instant::now())),
            Door::Opening(Some(_)) => Door::Closing(None),
            Door::Closing(Some(_)) => Door::Opening(None),
        }
    }

    // in milliseconds, 0 if completed
    fn get_time_to_complete_ms(&self) -> Option<u64> {
        match self {
            Door::Opening(Some(start)) => {
                let ellapsed = start.elapsed().as_millis();
                if ellapsed >= Self::OPENNING_DURATION_MS {
                    Some(0)
                } else {
                    Some((Self::OPENNING_DURATION_MS - ellapsed) as u64)
                }
            }
            Door::Closing(Some(start)) => {
                let ellapsed = start.elapsed().as_millis();
                if ellapsed >= Self::CLOSING_DURATION_MS {
                    Some(0)
                } else {
                    Some((Self::CLOSING_DURATION_MS - ellapsed) as u64)
                }
            }
            _ => None,
        }
    }

    fn update_state(&mut self) {
        debug!("Updating state {:?}", self);
        match self {
            Door::Opening(Some(start)) => {
                if start.elapsed().as_millis() >= Self::OPENNING_DURATION_MS {
                    debug!("Door opened");
                    *self = Door::Open;
                }
            }
            Door::Closing(Some(start)) => {
                if start.elapsed().as_millis() >= Self::CLOSING_DURATION_MS {
                    debug!("Door closed");
                    *self = Door::Closed;
                }
            }
            _ => (),
        }
    }

    fn is_open(&self) -> bool {
        !matches!(self, Door::Closed)
    }
}

impl ExpirableTrait<u64> for Door {
    fn ttl(&self) -> Option<u64> {
        self.get_time_to_complete_ms()
    }
}

pub struct GarageController {
    left_door: Door,   // RL1, IN3
    right_door: Door,  // RL2, IN4
    gate_closed: bool, // IN2
}

impl Default for GarageController {
    fn default() -> Self {
        Self {
            left_door: Door::default(),
            right_door: Door::default(),
            gate_closed: true,
        }
    }
}

impl Behavior for GarageController {
    fn on_command(
        &mut self,
        endpoint: &ct::Endpoint,
        payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<ct::ErrorCode> {
        if endpoint == &ct::Endpoint::BoardControl {
            if let Ok(command) = class0::Command::try_from_raw(&payload) {
                if command.crl1 == Xps::PulseOn {
                    self.left_door.pulse_relay();
                }

                if command.crl2 == Xps::PulseOn {
                    self.right_door.pulse_relay();
                }

                Some(ct::ErrorCode::Ok)
            } else {
                Some(ct::ErrorCode::Eframe)
            }
        } else {
            None
        }
    }

    fn on_telemetry(&mut self, endpoint: &ct::Endpoint) -> Option<Vec<u8>> {
        if endpoint == &ct::Endpoint::BoardControl {
            let mut telemetry = class0::Telemetry::default();

            telemetry.in1 = true; // nc
            telemetry.in2 = self.gate_closed;
            telemetry.in3 = self.left_door.is_open();
            telemetry.in4 = self.right_door.is_open();

            telemetry.temp_in = Temperature::random();
            telemetry.temp_out = [
                Temperature::random(),
                Temperature::INVALID,
                Temperature::INVALID,
            ];

            Some(telemetry.to_raw_vec())
        } else {
            None
        }
    }

    fn get_remaining_to_event_ms(&self) -> Option<u64> {
        [&self.left_door, &self.right_door].iter().ttl()
    }

    fn process(&mut self) -> Option<ct::Endpoint> {
        self.left_door.update_state();
        self.right_door.update_state();
        Some(ct::Endpoint::BoardControl)
    }
}
