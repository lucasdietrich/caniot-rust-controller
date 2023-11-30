use crate::{
    caniot::{self},
    controller::{ManagedDeviceError},
};

#[derive(Default)]
pub struct DemoNode {
    active: bool,
}

impl DemoNode {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}