use as_any::Downcast;
use rocket::Request;

use crate::{
    caniot::{self, emu::Device, RequestData, Response, ResponseData},
    controller::{
        ControllerAPI, ControllerError, DeviceActionResultTrait, DeviceActionTrait, DeviceError,
        DeviceProcessOutput, DeviceTrait,
    },
};

#[derive(Default, Debug)]
pub struct DemoController {
    active: bool,
}

impl DemoController {
    pub fn get_active(&self) -> Result<DeviceProcessOutput<DemoAction>, DeviceError> {
        println!("DemoNode::get_active() -> {}", self.active);

        Ok(DeviceProcessOutput::build_action_result(
            DemoActionResult::Active(self.active),
        ))
    }

    pub fn set_active(
        &mut self,
        active: bool,
    ) -> Result<DeviceProcessOutput<DemoAction>, DeviceError> {
        println!("DemoNode::set_active({})", active);

        self.active = active;

        let result = DeviceProcessOutput {
            requests: vec![RequestData::Command {
                endpoint: caniot::Endpoint::ApplicationDefault,
                payload: vec![active as u8],
            }],
            action_result: Some(DemoActionResult::Active(active)),
            ..Default::default()
        };

        Ok(result)
    }
}

impl DeviceTrait for DemoController {
    type Action = DemoAction;

    fn handle_action(
        &mut self,
        action: &DemoAction,
    ) -> Result<DeviceProcessOutput<DemoAction>, DeviceError> {
        println!("DemoNode::handle_action({:?})", action);
        match action {
            DemoAction::GetActive => self.get_active(),
            DemoAction::Activate => self.set_active(true),
            DemoAction::Deactivate => self.set_active(false),
            DemoAction::SetActive(active) => self.set_active(*active),
        }
    }

    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutput<DemoAction>, DeviceError> {
        if let caniot::ResponseData::Telemetry {
            endpoint: _,
            payload,
        } = frame
        {
            if payload.len() >= 1 {
                println!("DemoNode::handle_frame({:?})", payload[0] != 0);
                self.active = payload[0] != 0;
            }
        }
        Ok(DeviceProcessOutput::default())
    }

    fn process(&mut self) -> Result<DeviceProcessOutput<DemoAction>, DeviceError> {
        let mut result = DeviceProcessOutput::default();
        result.request_process_in_s(5);
        println!("DemoNode::process()");
        Ok(result)
    }
}

#[derive(Debug)]
pub enum DemoAction {
    GetActive,
    Activate,
    Deactivate,
    SetActive(bool),
}

#[derive(Debug)]
pub enum DemoActionResult {
    Active(bool),
}

impl DeviceActionTrait for DemoAction {
    type Result = DemoActionResult;
}

impl DeviceActionResultTrait for DemoActionResult {}
