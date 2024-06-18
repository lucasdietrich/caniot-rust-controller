use crate::{
    caniot::{self, class0::Class0, class1::Class1, AsPayload, SysCtrl, Temperature},
    utils::expirable::ExpirableTrait,
};

use super::Device;

// TODO implement default behavior
// errorcode not implemented yet
pub trait Behavior: Send + Sync {
    // Initialize the behavior context
    fn set_did(&mut self, _did: &caniot::DeviceId) {}

    // Handlers
    fn on_telemetry(&mut self, _endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        None
    }

    // Called when a command is received
    //
    // # Arguments
    // * `endpoint` - The endpoint of the command
    // * `payload` - The payload of the command
    // * `terminale` - A flag to indicate if the command is terminal (i.e. no further behavior command handler should be called)
    fn on_command(
        &mut self,
        _endpoint: &caniot::Endpoint,
        _payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<caniot::ErrorCode> {
        None
    }

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<caniot::ErrorCode> {
        None
    }

    fn process(&mut self) -> Option<caniot::Endpoint> {
        None
    }

    // time in milliseconds
    fn get_remaining_to_event_ms(&self) -> Option<u64> {
        None
    }
}

impl ExpirableTrait<u64> for Box<dyn Behavior> {
    fn ttl(&self) -> Option<u64> {
        self.get_remaining_to_event_ms()
    }
}

impl Device {
    pub fn add_behavior(&mut self, mut behavior: Box<dyn Behavior>) {
        behavior.set_did(&self.did);
        self.behavior.push(behavior);
    }
}

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
        _terminate: &mut bool,
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
        _terminate: &mut bool,
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
        _terminate: &mut bool,
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

/// Board control behavior
///
/// - Count hardware reset command
#[derive(Default)]
pub struct BoardControlBehavior {
    pub did: Option<caniot::DeviceId>,
    pub reset_count: u32,
}

impl Behavior for BoardControlBehavior {
    fn set_did(&mut self, did: &caniot::DeviceId) {
        self.did.replace(*did);
    }

    fn on_command(
        &mut self,
        _endpoint: &caniot::Endpoint,
        payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<caniot::ErrorCode> {
        if let Ok(sys_ctrl) = SysCtrl::try_from_raw(&payload) {
            debug!(
                "BoardControlBehavior: {:?} received for {:?}",
                sys_ctrl, self.did
            );

            if sys_ctrl.hardware_reset {
                self.reset_count += 1;
                debug!(
                    "BoardControlBehavior: hardware reset receiver for {:?} (count: {})",
                    self.did, self.reset_count
                );
            }

            Some(caniot::ErrorCode::Ok)
        } else {
            None
        }
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
        if !did.is::<Class0>() {
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

            Some(telemetry.to_raw_vec())
        } else {
            None
        }
    }

    fn on_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        _payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<caniot::ErrorCode> {
        if endpoint == &caniot::Endpoint::BoardControl {
            Some(caniot::ErrorCode::Enimpl)
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Class1Behavior {}

impl Behavior for Class1Behavior {
    fn set_did(&mut self, did: &caniot::DeviceId) {
        if !did.is::<Class1>() {
            panic!("Class1Behavior is only for class 1 devices");
        }
    }

    fn on_telemetry(&mut self, endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        if endpoint == &caniot::Endpoint::BoardControl {
            let mut telemetry = caniot::class1::Telemetry::default();

            for io in telemetry.ios.iter_mut() {
                *io = rand::random();
            }

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

    fn on_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        _payload: Vec<u8>,
        _terminate: &mut bool,
    ) -> Option<caniot::ErrorCode> {
        if endpoint == &caniot::Endpoint::BoardControl {
            Some(caniot::ErrorCode::Ok)
        } else {
            None
        }
    }
}
