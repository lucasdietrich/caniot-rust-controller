use as_any::Downcast;

use crate::{
    caniot::{self, Response},
    controller::{
        ControllerAPI, ControllerError, DeviceActionTrait, DeviceError, DeviceResult, DeviceTrait,
        LDevice, LDeviceTrait, LManagedDeviceError,
    },
};

#[derive(Default, Debug)]
pub struct DemoController {
    active: bool,
}

impl LDeviceTrait for DemoController {
    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<(), LManagedDeviceError> {
        println!("DemoNode::handle_frame({:?})", frame);
        Ok(())
    }
}

impl DemoController {
    pub fn get_active(&self) -> bool {
        println!("DemoNode::get_active() -> {}", self.active);
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        println!("DemoNode::set_active({})", active);
        self.active = active;
    }
}

impl DeviceTrait for DemoController {
    type Action = DemoAction;

    fn handle_action(&mut self, action: &DemoAction) -> Result<DeviceResult, DeviceError> {
        match action {
            DemoAction::Activate => {
                self.set_active(true);
            }
            DemoAction::Deactivate => {
                self.set_active(false);
            }
            DemoAction::SetActive(active) => {
                self.set_active(*active);
            }
        }
        Ok(DeviceResult::default())
    }

    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<DeviceResult, DeviceError> {
        Ok(DeviceResult::default())
    }

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        Ok(DeviceResult::default())
    }
}

#[derive(Debug)]
pub enum DemoAction {
    Activate,
    Deactivate,
    SetActive(bool),
}

impl DeviceActionTrait for DemoAction {}
