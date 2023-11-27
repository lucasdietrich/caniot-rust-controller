use log::info;
use tonic::{
    transport::{Error as GrpcError, Server},
    Code, Request, Response, Status,
};

use model::can_controller_server::{CanController, CanControllerServer};
use model::{Status as CommandStatus};
use model::*;

pub mod model {
    tonic::include_proto!("legacy");
}

use serde::{Deserialize, Serialize};

use crate::{shared::SharedHandle, controller, caniot};

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
        let message = request.into_inner();
        let command = GarageDoorCommand::try_from(message.command)
            .unwrap_or(GarageDoorCommand::CommandUnspecified);
        let (left, right)= match command {
            GarageDoorCommand::CommandUnspecified => (false, false),
            GarageDoorCommand::CommandAll => (true, true),
            GarageDoorCommand::CommandLeft => (true, false),
            GarageDoorCommand::CommandRight => (false, true),
        };

        let handle = self.shared.controller_handle.get_garage_handle();
        let status = match handle.send_command(left, right).await {
            Ok(_response) => CommandStatus::Ok,
            Err(controller::ControllerError::Timeout) => CommandStatus::Timeout,
            Err(_error) => CommandStatus::Nok,
        };
        
        Ok(Response::new(CommandResponse {
            status: status.into()
        }))
    }

    async fn get_alarm(
        &self,
        request: Request<AlarmCommand>,
    ) -> Result<Response<CommandResponse>, Status>{
        todo!();
    }

    async fn send_alarm(
        &self,
        request: Request<AlarmCommand>,
    ) -> Result<Response<CommandResponse>, Status>{
        todo!();
    }

    async fn set_heaters(
        &self,
        request: Request<HeatersCommand>,
    ) -> Result<Response<CommandResponse>, Status>{
        todo!();
    }

    async fn request_telemetry(
        &self,
        request: Request<TelemetryTarget>,
    ) -> Result<Response<Empty>, Status>{
        todo!();
    }

    async fn command_device(
        &self,
        request: Request<BoardLevelCommand>,
    ) -> Result<Response<CommandResponse>, Status>{
        todo!();
    }

    async fn read_attribute(
        &self,
        request: Request<AttributeRequest>,
    ) -> Result<Response<AttributeResponse>, Status>{
        todo!();
    }

    async fn write_attribute(
        &self,
        request: Request<AttributeRequest>,
    ) -> Result<Response<AttributeResponse>, Status>{
        todo!();
    }

    async fn reset(
        &self,
        request: Request<DeviceId>,
    ) -> Result<Response<CommandResponse>, Status>{
        todo!();
    }

    async fn reset_factory_defaults(
        &self,
        request: Request<DeviceId>,
    ) -> Result<Response<CommandResponse>, Status>{
        todo!();
    }

    async fn get_devices(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Devices>, Status>{
        todo!();
    }

    async fn get_device(
        &self,
        request: Request<DeviceId>,
    ) -> Result<Response<Device>, Status>{
        todo!();
    }
}


pub fn get_legacy_caniot_controller(shared: SharedHandle) -> CanControllerServer<LegacyCaniotController> {
    CanControllerServer::new(LegacyCaniotController {
        shared,
    })
}
