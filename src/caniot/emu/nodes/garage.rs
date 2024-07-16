use std::time::{Duration, Instant};

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
    pub const OPENNING_DURATION: Duration = Duration::from_millis(10_000);
    pub const CLOSING_DURATION: Duration = Duration::from_millis(10_000);

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
    fn get_time_to_complete_ms(&self, now: &Instant) -> Option<Duration> {
        match self {
            Door::Opening(Some(start)) => {
                let ellapsed = *now - *start;
                if ellapsed >= Self::OPENNING_DURATION {
                    Some(Duration::ZERO)
                } else {
                    Some(Self::OPENNING_DURATION - ellapsed)
                }
            }
            Door::Closing(Some(start)) => {
                let ellapsed = *now - *start;
                if ellapsed >= Self::CLOSING_DURATION {
                    Some(Duration::ZERO)
                } else {
                    Some(Self::CLOSING_DURATION - ellapsed)
                }
            }
            _ => None,
        }
    }

    fn update_state(&mut self, now: &Instant) {
        debug!("Updating state {:?}", self);
        match self {
            Door::Opening(Some(start)) => {
                if *now - *start >= Self::OPENNING_DURATION {
                    debug!("Door opened");
                    *self = Door::Open;
                }
            }
            Door::Closing(Some(start)) => {
                if *now - *start >= Self::CLOSING_DURATION {
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

impl ExpirableTrait<Duration> for Door {
    const ZERO: Duration = Duration::ZERO;
    type Instant = Instant;

    fn ttl(&self, now: &Instant) -> Option<Duration> {
        self.get_time_to_complete_ms(now)
    }
}

pub struct GarageController {
    left_door: Door,  // RL1, IN3
    right_door: Door, // RL2, IN4
    gate_open: bool,  // IN2
}

impl Default for GarageController {
    fn default() -> Self {
        Self {
            left_door: Door::default(),
            right_door: Door::default(),
            gate_open: false,
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
            telemetry.in2 = self.gate_open;
            telemetry.in3 = self.left_door.is_open();
            telemetry.in4 = self.right_door.is_open();

            telemetry.temp_in = Temperature::random_full_range();
            telemetry.temp_out = [
                Temperature::INVALID,
                Temperature::INVALID,
                Temperature::INVALID,
            ];

            Some(telemetry.to_raw_vec())
        } else {
            None
        }
    }

    fn get_remaining_to_event(&self, now: &Instant) -> Option<Duration> {
        [&self.left_door, &self.right_door].iter().ttl(now)
    }

    fn process(&mut self, now: &Instant) -> Option<ct::Endpoint> {
        self.left_door.update_state(now);
        self.right_door.update_state(now);
        Some(ct::Endpoint::BoardControl)
    }
}
