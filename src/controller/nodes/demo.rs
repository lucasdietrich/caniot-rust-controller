use crate::{
    caniot::{self},
    controller::{ManagedDeviceTrait, ManagedDeviceError, Device},
};

#[derive(Default)]
pub struct DemoNode {
    active: bool,
}

impl ManagedDeviceTrait for DemoNode {
    // type Error = ManagedDeviceError;

    fn handle_frame(&mut self, frame: &caniot::Response) -> Result<(), ManagedDeviceError> {
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

impl DemoNode {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}