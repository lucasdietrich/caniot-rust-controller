use crate::caniot::{self, traits::Class, Temperature};

use super::Device;

// TODO implement default behavior
// errorcode not implemented yet
pub trait Behavior: Send + Sync {
    // Initialize the behavior context
    fn set_did(&mut self, did: &caniot::DeviceId) {}

    // Handlers
    fn on_telemetry(&mut self, endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        None
    }

    fn on_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        None
    }

    fn on_read_attribute(&mut self, key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, key: u16, value: u32) -> Option<caniot::ErrorCode> {
        None
    }
}

impl Device {
    pub fn add_behavior(&mut self, mut behavior: Box<dyn Behavior>) {
        behavior.set_did(&self.did);
        self.behavior.push(behavior);
    }
}

/// Default behavior
/// - Nothing handled
#[derive(Default)]
pub struct DefaultBehavior();

impl Behavior for DefaultBehavior {}

/// Counter behavior
/// - Count up on command and return the count on telemetry
#[derive(Default)]
pub struct CounterBehavior {
    count: u32,
}

impl Behavior for CounterBehavior {
    fn set_did(&mut self, _did: &caniot::DeviceId) {}

    fn on_telemetry(&mut self, _endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        Some(self.count.to_be_bytes()[..4].to_vec())
    }

    fn on_command(
        &mut self,
        _endpoint: &caniot::Endpoint,
        _payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        self.count += 1;
        Some(caniot::ErrorCode::Ok)
    }
}

/// Echo behavior
/// - Echo the last command on telemetry
#[derive(Default)]
pub struct EchoBehavior {
    last_command: Option<Vec<u8>>,
}

impl Behavior for EchoBehavior {
    fn set_did(&mut self, _did: &caniot::DeviceId) {}

    fn on_telemetry(&mut self, _endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        if let Some(last_command) = self.last_command.take() {
            Some(last_command)
        } else {
            None
        }
    }

    fn on_command(
        &mut self,
        _endpoint: &caniot::Endpoint,
        payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        self.last_command = Some(payload);
        Some(caniot::ErrorCode::Ok)
    }
}

/// Random behavior
/// - Return random telemetry
#[derive(Default)]
pub struct RandomBehavior();

impl Behavior for RandomBehavior {
    fn set_did(&mut self, _did: &caniot::DeviceId) {}

    fn on_telemetry(&mut self, _endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        Some((0..4).map(|_| rand::random::<u8>()).collect())
    }

    fn on_command(
        &mut self,
        _endpoint: &caniot::Endpoint,
        _payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        Some(caniot::ErrorCode::Ok)
    }

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        Some(rand::random::<u32>())
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<caniot::ErrorCode> {
        Some(caniot::ErrorCode::Ok)
    }
}

#[derive(Default)]
pub struct Class0Behavior {
    pub oc1: bool,
    pub oc2: bool,
    pub rl1: bool,
    pub rl2: bool,
}

impl Behavior for Class0Behavior {
    fn set_did(&mut self, did: &caniot::DeviceId) {
        if did.class != 0 {
            panic!("Class0Behavior is only for class 0 devices");
        }
    }

    fn on_telemetry(&mut self, endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        if endpoint == &caniot::Endpoint::BoardControl {
            let mut telemetry = caniot::class0::Telemetry::default();

            telemetry.in1 = rand::random();
            telemetry.in2 = rand::random();
            telemetry.in3 = rand::random();
            telemetry.in4 = rand::random();

            telemetry.oc1 = self.oc1;
            telemetry.oc2 = self.oc2;
            telemetry.rl1 = self.rl1;
            telemetry.rl2 = self.rl2;

            telemetry.temp_in = Temperature::random();
            telemetry.temp_out = [
                Temperature::random(),
                Temperature::INVALID,
                Temperature::INVALID,
            ];

            Some(caniot::BlcClassTelemetry::Class0(telemetry).into())
        } else {
            None
        }
    }

    fn on_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        if endpoint == &caniot::Endpoint::BoardControl {
            Some(caniot::ErrorCode::Ok)
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Class1Behavior {}

impl Behavior for Class1Behavior {
    fn set_did(&mut self, did: &caniot::DeviceId) {
        if did.class != 1 {
            panic!("Class1Behavior is only for class 1 devices");
        }
    }

    fn on_telemetry(&mut self, endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        if endpoint == &caniot::Endpoint::BoardControl {
            let mut telemetry = caniot::class1::Telemetry::default();

            for (i, io) in telemetry.ios.iter_mut().enumerate() {
                *io = rand::random();
            }

            telemetry.temp_in = Temperature::random();
            telemetry.temp_out = [
                Temperature::random(),
                Temperature::INVALID,
                Temperature::INVALID,
            ];

            Some(caniot::BlcClassTelemetry::Class1(telemetry).into())
        } else {
            None
        }
    }

    fn on_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        if endpoint == &caniot::Endpoint::BoardControl {
            Some(caniot::ErrorCode::Ok)
        } else {
            None
        }
    }
}
