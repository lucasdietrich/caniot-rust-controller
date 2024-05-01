use log::info;

use crate::{
    caniot::{self, HeatingControllerCommand, HeatingControllerTelemetry, HeatingMode},
    controller::{
        DeviceActionResultTrait, DeviceActionTrait, DeviceActionVerdict, DeviceProcessContext,
        DeviceTrait, DeviceVerdict,
    },
};

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

impl DeviceActionTrait for HeaterAction {
    type Result = HeaterStatus;
}

impl DeviceActionResultTrait for HeaterStatus {}

impl DeviceTrait for HeatersController {
    type Action = HeaterAction;

    fn handle_action(
        &mut self,
        action: &Self::Action,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceActionVerdict<Self::Action>, crate::controller::DeviceError> {
        let verdict = match action {
            HeaterAction::GetStatus => DeviceActionVerdict::ActionResult(self.status.clone()),
            HeaterAction::SetStatus(heaters) => {
                let command = HeatingControllerCommand {
                    modes: [heaters[0], heaters[1], heaters[2], heaters[3]],
                };

                DeviceActionVerdict::ActionPendingOn(caniot::RequestData::Command {
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
    ) -> Result<<Self::Action as DeviceActionTrait>::Result, crate::controller::DeviceError> {
        Ok(self.status.clone())
    }

    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdict, crate::controller::DeviceError> {
        match &frame {
            &caniot::ResponseData::Telemetry { endpoint, payload }
                if endpoint == &caniot::Endpoint::ApplicationDefault =>
            {
                // interpret the payload as telemetry
                let telemetry = HeatingControllerTelemetry::try_from(payload.as_ref())?;

                // update internal state
                self.status.heaters = telemetry.modes;
                self.status.power_status = telemetry.power_status;
            }
            _ => {}
        };

        Ok(DeviceVerdict::default())
    }
}
