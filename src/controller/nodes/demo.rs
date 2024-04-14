use crate::{
    caniot::{self, datatypes::Class0Command, Response},
    controller::{ControllerAPI, ControllerError, Device, DeviceTrait, ManagedDeviceError},
};

#[derive(Default)]
pub struct DemoNode {
    active: bool,
}

impl DeviceTrait for DemoNode {
    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<(), ManagedDeviceError> {
        println!("DemoNode::handle_frame({:?})", frame);
        Ok(())
    }
}

impl DemoNode {
    pub fn get_active(&self) -> bool {
        println!("DemoNode::get_active() -> {}", self.active);
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        println!("DemoNode::set_active({})", active);
        self.active = active;
    }
}

pub enum DemoAction {
    Activate,
    Deactivate,
    SetActive(bool),
}

impl Device<DemoNode> {
    pub async fn handle_action(
        &mut self,
        api: &mut dyn ControllerAPI,
        command: DemoAction,
    ) -> Result<Option<Response>, ControllerError> {
        match command {
            DemoAction::Activate => {
                self.inner.set_active(true);
            }
            DemoAction::Deactivate => {
                self.inner.set_active(false);
            }
            DemoAction::SetActive(active) => {
                self.inner.set_active(active);
            }
        }

        Ok(None)
    }
}
