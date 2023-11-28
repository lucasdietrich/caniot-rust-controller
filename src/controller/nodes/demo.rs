use std::default;

use crate::{
    caniot::{self},
    controller::{DeviceTrait, DeviceError, ManagedDevice},
};

#[derive(Default)]
pub struct DemoNode {
    active: bool,
}

impl DeviceTrait for DemoNode {
    type Error = DeviceError;

    fn handle_frame(&mut self, frame: &caniot::Response) -> Result<(), Self::Error> {
        match &frame.data {
            caniot::ResponseData::Telemetry { payload , ..} => {
                if payload.len() >= 1 && payload[0] == 0x01 {
                    self.active = true;
                }
            },
            _ => {}
        }

        Ok(())
    }
}