use as_any::Downcast;
use rocket::Request;

use crate::{
    caniot::{self, emu::Device, RequestData, Response, ResponseData},
    controller::{
        ControllerAPI, ControllerError, DeviceActionTrait, DeviceError, DeviceResult, DeviceTrait,
    },
};

#[derive(Default, Debug)]
pub struct DemoController {
    active: bool,
}

impl DemoController {
    pub fn get_active(&self) -> bool {
        println!("DemoNode::get_active() -> {}", self.active);
        self.active
    }

    pub fn set_active(&mut self, active: bool) -> Result<DeviceResult, DeviceError> {
        println!("DemoNode::set_active({})", active);
        self.active = active;
        Ok(DeviceResult::from_request_data(RequestData::Command {
            endpoint: caniot::Endpoint::ApplicationDefault,
            payload: vec![active as u8],
        }))
    }
}

impl DeviceTrait for DemoController {
    type Action = DemoAction;

    fn handle_action(&mut self, action: &DemoAction) -> Result<DeviceResult, DeviceError> {
        println!("DemoNode::handle_action({:?})", action);
        match action {
            DemoAction::Activate => self.set_active(true),
            DemoAction::Deactivate => self.set_active(false),
            DemoAction::SetActive(active) => self.set_active(*active),
        }
    }

    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<DeviceResult, DeviceError> {
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
        Ok(DeviceResult::default())
    }

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        let mut result = DeviceResult::default();
        result.request_process_in_s(5);
        println!("DemoNode::process()");
        Ok(result)
    }
}

#[derive(Debug)]
pub enum DemoAction {
    Activate,
    Deactivate,
    SetActive(bool),
}

impl DeviceActionTrait for DemoAction {}
