use super::super::Behavior;
use crate::caniot::{self as ct};

pub struct DemoController {
    status: bool,
}

impl DemoController {
    pub fn new() -> Self {
        Self { status: true }
    }
}

impl Behavior for DemoController {
    fn on_command(&mut self, _endpoint: &ct::Endpoint, payload: Vec<u8>) -> Option<ct::ErrorCode> {
        if payload.len() >= 1 {
            if payload[0] == 0 {
                self.status = false;
            } else {
                self.status = true;
            }
        }
        None
    }

    fn on_telemetry(&mut self, _endpoint: &ct::Endpoint) -> Option<Vec<u8>> {
        let telemetry = vec![self.status as u8];
        return Some(telemetry);
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
