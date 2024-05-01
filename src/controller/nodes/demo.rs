use as_any::Downcast;
use rocket::Request;

use crate::{
    caniot::{self, emu::Device, RequestData, Response, ResponseData},
    controller::{
        ControllerAPI, ControllerError, DeviceActionResultTrait, DeviceActionTrait,
        DeviceActionVerdict, DeviceError, DeviceProcessContext, DeviceTrait, DeviceVerdict,
    },
};

#[derive(Default, Debug)]
pub struct DemoController {
    active: bool,
}

impl DemoController {
    pub fn get_active(&self) -> Result<DeviceActionVerdict<DemoAction>, DeviceError> {
        Ok(DeviceActionVerdict::ActionResult(DemoActionResult::Active(
            self.active,
        )))
    }

    pub fn set_active(
        &mut self,
        active: bool,
    ) -> Result<DeviceActionVerdict<DemoAction>, DeviceError> {
        self.active = active;

        Ok(DeviceActionVerdict::ActionPendingOn(RequestData::Command {
            endpoint: caniot::Endpoint::ApplicationDefault,
            payload: vec![active as u8],
        }))
    }
}

impl DeviceTrait for DemoController {
    type Action = DemoAction;

    fn handle_action(
        &mut self,
        action: &DemoAction,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceActionVerdict<DemoAction>, DeviceError> {
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
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdict, DeviceError> {
        if let caniot::ResponseData::Telemetry {
            endpoint: _,
            payload,
        } = frame
        {
            if payload.len() >= 1 {
                self.active = payload[0] != 0;
            }
        }

        Ok(DeviceVerdict::default())
    }

    fn process(&mut self, ctx: &mut DeviceProcessContext) -> Result<DeviceVerdict, DeviceError> {
        ctx.request_process_in_s(5);

        Ok(DeviceVerdict::default())
    }
}

#[derive(Debug)]
pub enum DemoAction {
    GetActive,
    Activate,
    Deactivate,
    SetActive(bool),
}

#[derive(Debug, Clone)]
pub enum DemoActionResult {
    Active(bool),
}

impl DeviceActionTrait for DemoAction {
    type Result = DemoActionResult;
}

impl DeviceActionResultTrait for DemoActionResult {}
