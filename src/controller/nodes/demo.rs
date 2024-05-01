use as_any::Downcast;
use rocket::Request;

use crate::{
    caniot::{self, emu::Device, RequestData, Response, ResponseData},
    controller::{
        ActionResultTrait, ActionTrait, ActionVerdict, ControllerError, DeviceError, DeviceTrait,
        ProcessContext, Verdict,
    },
};

#[derive(Default, Debug)]
pub struct DemoController {
    active: bool,
}

impl DemoController {
    pub fn get_active(&self) -> Result<ActionVerdict<DemoAction>, DeviceError> {
        Ok(ActionVerdict::ActionResult(DemoActionResult::Active(
            self.active,
        )))
    }

    pub fn set_active(&mut self, active: bool) -> Result<ActionVerdict<DemoAction>, DeviceError> {
        self.active = active;

        Ok(ActionVerdict::ActionPendingOn(RequestData::Command {
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
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<DemoAction>, DeviceError> {
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
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        if let caniot::ResponseData::Telemetry {
            endpoint: _,
            payload,
        } = frame
        {
            if payload.len() >= 1 {
                self.active = payload[0] != 0;
            }
        }

        Ok(Verdict::default())
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> Result<Verdict, DeviceError> {
        ctx.request_process_in_s(5);

        Ok(Verdict::default())
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

impl ActionTrait for DemoAction {
    type Result = DemoActionResult;
}

impl ActionResultTrait for DemoActionResult {}
