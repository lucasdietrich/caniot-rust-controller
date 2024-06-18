use crate::{
    caniot::{self, BoardClassTelemetry, HeatingMode, Response},
    controller::{
        ActionResultTrait, ActionTrait, ActionVerdict, DeviceControllerInfos,
        DeviceControllerTrait, ProcessContext, Verdict,
    },
};

use super::types::{HeatingControllerCommand, HeatingControllerTelemetry};

#[derive(Debug, Default, Clone)]
pub struct HeatersController {
    pub status: HeaterStatus,
}

#[derive(Debug, Default, Clone)]
pub struct HeaterStatus {
    pub heaters: [HeatingMode; 4],
    pub power_status: bool,
}

#[derive(Debug)]
pub enum HeaterAction {
    GetStatus,
    SetStatus(Vec<HeatingMode>),
}

impl ActionTrait for HeaterAction {
    type Result = HeaterStatus;
}

impl ActionResultTrait for HeaterStatus {}

impl DeviceControllerTrait for HeatersController {
    type Action = HeaterAction;

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new("Heaters Controller")
    }

    fn handle_action(
        &mut self,
        action: &Self::Action,
        _ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<Self::Action>, crate::controller::DeviceError> {
        let verdict = match action {
            HeaterAction::GetStatus => ActionVerdict::ActionResult(self.status.clone()),
            HeaterAction::SetStatus(heaters) => {
                let command = HeatingControllerCommand {
                    modes: [heaters[0], heaters[1], heaters[2], heaters[3]],
                };

                ActionVerdict::ActionPendingOn(caniot::RequestData::Command {
                    endpoint: caniot::Endpoint::ApplicationDefault,
                    payload: command.into(),
                })
            }
        };

        Ok(verdict)
    }

    fn handle_action_result(
        &self,
        _action: &Self::Action,
        _completed_by: Response,
    ) -> Result<<Self::Action as ActionTrait>::Result, crate::controller::DeviceError> {
        Ok(self.status.clone())
    }

    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
        _as_class_blc: &Option<BoardClassTelemetry>,
        _ctx: &mut ProcessContext,
    ) -> Result<Verdict, crate::controller::DeviceError> {
        match &frame {
            &caniot::ResponseData::Telemetry { endpoint, payload }
                if endpoint == &caniot::Endpoint::ApplicationDefault =>
            {
                // interpret the payload as telemetry
                let telemetry = HeatingControllerTelemetry::try_from(payload)?;

                // update internal state
                self.status.heaters = telemetry.modes;
                self.status.power_status = telemetry.power_status;
            }
            _ => {}
        };

        Ok(Verdict::default())
    }
}
