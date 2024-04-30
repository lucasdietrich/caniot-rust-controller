use log::info;

use crate::{
    caniot::{self, HeatingControllerTelemetry, HeatingMode},
    controller::{DeviceActionResultTrait, DeviceActionTrait, DeviceProcessOutput, DeviceTrait},
};

#[derive(Debug, Default, Clone)]
pub struct HeatersController {
    pub status: HeaterStatus,
    pub requested_status: HeaterStatus,
}

impl DeviceTrait for HeatersController {
    type Action = HeaterAction;

    fn handle_action(
        &mut self,
        action: &Self::Action,
    ) -> Result<DeviceProcessOutput<Self::Action>, crate::controller::DeviceError> {
        match action {
            HeaterAction::GetStatus => {}
            HeaterAction::SetStatus(heaters) => {
                for (i, heater) in heaters.iter().enumerate() {
                    self.requested_status.heaters[i] = *heater;
                }
            }
        };

        Ok(DeviceProcessOutput::new_action_result(self.status.clone()))
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
