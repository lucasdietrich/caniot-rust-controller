use log::info;
use tonic::{
    transport::{Error as GrpcError, Server},
    Code, Request, Response, Status,
};

use model::can_controller_server::{CanController, CanControllerServer};
use model::Status as CommandStatus;
use model::*;

pub mod model {
    tonic::include_proto!("legacy");
}

use serde::{Deserialize, Serialize};

use crate::{
    caniot::{self, DeviceId},
    controller::{self, ControllerError, DeviceAction, GarageDoorCommand, GarageNode},
    shared::SharedHandle,
};

#[derive(Debug)]
pub struct LegacyCaniotController {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl CanController for LegacyCaniotController {
    async fn send_garage(
        &self,
        request: Request<GarageCommand>,
    ) -> Result<Response<CommandResponse>, Status> {
        let request = request.into_inner();
        let request = model::GarageDoorCommand::try_from(request.command)
            .unwrap_or(model::GarageDoorCommand::CommandUnspecified);
        let (left, right) = match request {
            model::GarageDoorCommand::CommandUnspecified => (false, false),
            model::GarageDoorCommand::CommandAll => (true, true),
            model::GarageDoorCommand::CommandLeft => (true, false),
            model::GarageDoorCommand::CommandRight => (false, true),
        };
        let command = GarageDoorCommand {
            left_door_activate: left,
            right_door_activate: right,
        };

        let result = self
            .shared
            .controller_handle
            .device_action(None, DeviceAction::Garage(command))
            .await;
        let status = match result {
            Ok(_) => model::Status::Ok,
            Err(ControllerError::Timeout) => model::Status::Timeout,
            Err(_) => model::Status::Nok,
        };

        Ok(Response::new(model::CommandResponse {
            status: status.into(),
        }))
    }

    async fn get_alarm(
        &self,
        request: Request<AlarmCommand>,
    ) -> Result<Response<CommandResponse>, Status> {
        todo!();
    }

    async fn send_alarm(
        &self,
        request: Request<AlarmCommand>,
    ) -> Result<Response<CommandResponse>, Status> {
        todo!();
    }

    async fn set_heaters(
        &self,
        request: Request<HeatersCommand>,
    ) -> Result<Response<CommandResponse>, Status> {
        todo!();
    }

    async fn request_telemetry(
        &self,
        request: Request<TelemetryTarget>,
    ) -> Result<Response<Empty>, Status> {
        todo!();
    }

    async fn command_device(
        &self,
        request: Request<BoardLevelCommand>,
    ) -> Result<Response<CommandResponse>, Status> {
        todo!();
    }

    async fn read_attribute(
        &self,
        request: Request<AttributeRequest>,
    ) -> Result<Response<AttributeResponse>, Status> {
        todo!();
    }

    async fn write_attribute(
        &self,
        request: Request<AttributeRequest>,
    ) -> Result<Response<AttributeResponse>, Status> {
        todo!();
    }

    async fn reset(
        &self,
        request: Request<model::DeviceId>,
    ) -> Result<Response<CommandResponse>, Status> {
        todo!();
    }

    async fn reset_factory_defaults(
        &self,
        request: Request<model::DeviceId>,
    ) -> Result<Response<CommandResponse>, Status> {
        todo!();
    }

    async fn get_devices(
        &self,
        request: Request<model::Empty>,
    ) -> Result<Response<Devices>, Status> {
        todo!();
    }

    async fn get_device(
        &self,
        request: Request<model::DeviceId>,
    ) -> Result<Response<Device>, Status> {
        Ok(Response::new(Device {
            ..Default::default()
        }))
    }
}

pub fn get_legacy_caniot_controller(
    shared: SharedHandle,
) -> CanControllerServer<LegacyCaniotController> {
    CanControllerServer::new(LegacyCaniotController { shared })
}
