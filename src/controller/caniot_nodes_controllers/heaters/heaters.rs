use chrono::{DateTime, Utc};

use crate::{
    caniot::{self, BoardClassTelemetry, Endpoint, HeatingMode, Response},
    controller::{
        ActionResultTrait, ActionTrait, ActionVerdict, DeviceAlert, DeviceControllerInfos,
        DeviceControllerTrait, DeviceError, DeviceJobImpl, ProcessContext, Verdict,
    },
    ha::LOCATION_ATTIC,
};

use super::types::{HeatingControllerCommand, HeatingControllerTelemetry};

pub const HEATERS_ENDPOINT: Endpoint = Endpoint::ApplicationDefault;

#[derive(Debug, Default, Clone)]
pub struct HeatersController {
    pub status: HeaterStatus,

    // Monitor the number of received telemetry frames for the status endpoint "ApplicationDefault"
    pub status_telemetry_rx_count: usize,
    pub status_telemetry_req_sent: bool,
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
    type Job = ();
    type Config = ();

    fn get_infos(&self) -> DeviceControllerInfos {
        DeviceControllerInfos::new(
            "heaters",
            Some(LOCATION_ATTIC),
            Some("Chauffage lucas"),
            Some("heaters"),
        )
    }

    fn process_job(
        &mut self,
        job: &DeviceJobImpl<Self::Job>,
        _job_timestamp: &DateTime<Utc>,
        _ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        if job.is_device_add() {
            return Ok(Verdict::Request(caniot::RequestData::Telemetry {
                endpoint: HEATERS_ENDPOINT,
            }));
        }

        Ok(Verdict::default())
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
                    endpoint: HEATERS_ENDPOINT,
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
                if endpoint == &HEATERS_ENDPOINT =>
            {
                self.status_telemetry_rx_count += 1;

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

    fn get_alert(&self) -> Option<DeviceAlert> {
        if self.status_telemetry_rx_count == 0 {
            Some(
                DeviceAlert::new_warning("Etat du chauffage inconnu")
                    .with_description("Pas de télémesure reçue pour l'état du chauffage"),
            )
        } else if !self.status.power_status {
            Some(
                DeviceAlert::new_warning("Chauffage non alimenté")
                    .with_description("Pas de présence tension sur le chauffage"),
            )
        } else if self
            .status
            .heaters
            .iter()
            .any(|&mode| mode.heater_on(false))
        {
            Some(
                DeviceAlert::new_notification("Chauffage allumé")
                    .with_description("Au moins un chauffage est allumé"),
            )
        } else {
            None
        }
    }

    fn get_config(&self) -> &Self::Config {
        &()
    }
}
