use garage::GarageDoorCommand;

use tonic::{Request, Response, Result, Status};

use crate::{controller::garage, shared::SharedHandle};

use super::model_garage::{
    self as m,
    garage_service_server::{GarageService, GarageServiceServer},
};

#[derive(Debug)]
pub struct NgGarage {
    pub shared: SharedHandle,
}

impl Into<m::DoorState> for garage::DoorState {
    fn into(self) -> m::DoorState {
        match self {
            garage::DoorState::Open => m::DoorState::Open,
            garage::DoorState::Closed => m::DoorState::Closed,
            garage::DoorState::Moving(_) => m::DoorState::Open,
        }
    }
}

impl Into<m::DoorState> for bool {
    fn into(self) -> m::DoorState {
        if self {
            m::DoorState::Closed
        } else {
            m::DoorState::Open
        }
    }
}

impl NgGarage {
    fn garage_status_to_proto(&self, status: &Option<garage::GarageDoorStatus>) -> m::Status {
        if let Some(status) = status {
            m::Status {
                left_closed: Into::<m::DoorState>::into(status.left_door_status).into(),
                left_progress: status.left_door_status.progress().map(|p| p as u32),
                right_closed: Into::<m::DoorState>::into(status.right_door_status).into(),
                right_progress: status.right_door_status.progress().map(|p| p as u32),
                gate_closed: Into::<m::DoorState>::into(status.gate_open).into(),
            }
        } else {
            m::Status {
                left_closed: m::DoorState::Unknown.into(),
                left_progress: None,
                right_closed: m::DoorState::Unknown.into(),
                right_progress: None,
                gate_closed: m::DoorState::Unknown.into(),
            }
        }
    }
}

#[tonic::async_trait]
impl GarageService for NgGarage {
    async fn get_state(&self, _req: Request<()>) -> Result<Response<m::Status>, Status> {
        let api = self.shared.controller_handle.clone();
        let action = garage::GarageAction::GetStatus;

        let result = api
            .device_action_inner(None, action, None)
            .await
            .map_err(|e| Status::internal(format!("Error in get_state: {:?}", e)))?;

        Ok(Response::new(self.garage_status_to_proto(&result)))
    }

    async fn set_state(
        &self,
        req: Request<m::CommandMessage>,
    ) -> Result<Response<m::Status>, Status> {
        let api = self.shared.controller_handle.clone();
        let req = m::Command::try_from(req.into_inner().command).expect("Invalid Garage Command");
        let command = match req {
            m::Command::None => GarageDoorCommand::default(),
            m::Command::Left => GarageDoorCommand::OPEN_LEFT,
            m::Command::Right => GarageDoorCommand::OPEN_RIGHT,
            m::Command::All => GarageDoorCommand::OPEN_BOTH,
        };
        let action = garage::GarageAction::SetStatus(command);
        let result = api
            .device_action_inner(None, action, None)
            .await
            .map_err(|e| Status::internal(format!("Error in set_state: {:?} {:?}", command, e)))?;

        Ok(Response::new(self.garage_status_to_proto(&result)))
    }
}

pub fn get_ng_garage_server(shared: SharedHandle) -> GarageServiceServer<NgGarage> {
    GarageServiceServer::new(NgGarage { shared })
}
