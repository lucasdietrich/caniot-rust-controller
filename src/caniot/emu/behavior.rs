use crate::caniot;

use super::Device;

pub trait Behavior: Send + Sync {
    // Initialize the behavior context
    fn set_did(&mut self, did: &caniot::DeviceId);

    // Handlers
    fn on_telemetry(&mut self, endpoint: &caniot::Endpoint) -> Option<Vec<u8>>;
    fn on_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode>;
    fn on_read_attribute(&mut self, key: u16) -> Option<u32>;
    fn on_write_attribute(&mut self, key: u16, value: u32) -> Option<caniot::ErrorCode>;
}

impl Device {
    pub fn add_behavior(&mut self, mut behavior: Box<dyn Behavior>) {
        behavior.set_did(&self.did);
        self.behavior.push(behavior);
    }
}

#[derive(Default)]
pub struct DefaultBehavior();

impl Behavior for DefaultBehavior {
    fn set_did(&mut self, _did: &caniot::DeviceId) {}

    fn on_telemetry(&mut self, _endpoint: &caniot::Endpoint) -> Option<Vec<u8>> {
        None
    }

    fn on_command(
        &mut self,
        _endpoint: &caniot::Endpoint,
        _payload: Vec<u8>,
    ) -> Option<caniot::ErrorCode> {
        None
    }

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<caniot::ErrorCode> {
        None
    }
}

#[derive(Default)]
pub struct CounterBehavior {
    count: u32,
}

impl Behavior for CounterBehavior {
    fn set_did(&mut self, did: &caniot::DeviceId) {}

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

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<caniot::ErrorCode> {
        None
    }
}

#[derive(Default)]
pub struct EchoBehavior {
    last_command: Option<Vec<u8>>,
}

impl Behavior for EchoBehavior {
    fn set_did(&mut self, did: &caniot::DeviceId) {}

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

    fn on_read_attribute(&mut self, _key: u16) -> Option<u32> {
        None
    }

    fn on_write_attribute(&mut self, _key: u16, _value: u32) -> Option<caniot::ErrorCode> {
        None
    }
}

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
