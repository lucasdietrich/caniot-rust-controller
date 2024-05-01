use log::info;

use crate::{
    caniot::{self, HeatingControllerCommand, HeatingControllerTelemetry, HeatingMode},
    controller::{DeviceActionResultTrait, DeviceActionTrait, DeviceProcessOutput, DeviceTrait},
};

#[derive(Debug, Default, Clone)]
pub struct HeatersController {
    pub status: HeaterStatus,
}

impl DeviceTrait for HeatersController {
    type Action = HeaterAction;

    fn handle_action(
        &mut self,
        action: &Self::Action,
    ) -> Result<DeviceProcessOutput<Self::Action>, crate::controller::DeviceError> {
        let mut out = DeviceProcessOutput::default();

        match action {
            HeaterAction::GetStatus => {}
            HeaterAction::SetStatus(heaters) => {
                let command = HeatingControllerCommand {
                    modes: [heaters[0], heaters[1], heaters[2], heaters[3]],
                };

                out.add_request_data(caniot::RequestData::Command {
                    endpoint: caniot::Endpoint::ApplicationDefault,
                    payload: command.into(),
                });
            }
        };

        out.set_action_result(self.status.clone());

        Ok(out)
    }

    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutput<Self::Action>, crate::controller::DeviceError> {
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

        Ok(DeviceProcessOutput::default())
    }
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
