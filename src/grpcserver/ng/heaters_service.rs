use num_traits::{FromPrimitive, ToPrimitive};
use tonic::{Request, Response, Result, Status};

use crate::{
    caniot::HeatingMode,
    controller::{demo, heaters, DeviceAction},
    grpcserver::datetime_to_prost_timestamp,
    shared::SharedHandle,
};

use super::model_heaters::{
    self as m,
    heaters_service_server::{HeatersService, HeatersServiceServer},
};

#[derive(Debug)]
pub struct NgHeaters {
    pub shared: SharedHandle,
}

impl NgHeaters {
    fn heater_status_to_proto(&self, status: &heaters::HeaterStatus) -> m::Status {
        m::Status {
            heater: status
                .heaters
                .iter()
                .map(|h| h.to_i32().unwrap_or_default())
                .collect(),
            power_status: status.power_status,
        }
    }
}

#[tonic::async_trait]
impl HeatersService for NgHeaters {
    async fn get_state(&self, _req: Request<()>) -> Result<Response<m::Status>, Status> {
        let api = self.shared.controller_handle.clone();
        let action = heaters::HeaterAction::GetStatus;

        let result = api
            .device_action_inner(None, action)
            .await
            .map_err(|e| Status::internal(format!("Error in get_state: {:?}", e)))?;

        Ok(Response::new(self.heater_status_to_proto(&result)))
    }

    async fn set_state(&self, req: Request<m::Command>) -> Result<Response<m::Status>, Status> {
        let api = self.shared.controller_handle.clone();
        let heaters = req.into_inner().heater;
        let heaters = heaters
            .iter()
            .map(|h| HeatingMode::from_i32(*h).unwrap_or_default())
            .collect();

        let action = heaters::HeaterAction::SetStatus(heaters);

        let result = api
            .device_action_inner(None, action)
            .await
            .map_err(|e| Status::internal(format!("Error in set_state: {:?}", e)))?;

        Ok(Response::new(self.heater_status_to_proto(&result)))
    }
}

pub fn get_ng_heaters_server(shared: SharedHandle) -> HeatersServiceServer<NgHeaters> {
    HeatersServiceServer::new(NgHeaters { shared })
}
