use log::info;
use tonic::{
    transport::{Error as GrpcError, Server},
    Code, Request, Response, Status,
};

use model::caniot_controller_server::{CaniotController, CaniotControllerServer};
use model::*;

pub mod model {
    tonic::include_proto!("ng");
}

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::shared::SharedHandle;

#[derive(Debug)]
pub struct NgCaniotController {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl CaniotController for NgCaniotController {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }

    async fn hello_empty(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }

    async fn request_telemetry(
        &self,
        request: Request<TelemetryRequest>,
    ) -> Result<Response<TelemetryResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = TelemetryResponse {
            message: format!("Hello!"),
        };

        Ok(Response::new(response))
    }
}

pub fn get_ng_caniot_controller(
    shared: SharedHandle,
) -> CaniotControllerServer<NgCaniotController> {
    CaniotControllerServer::new(NgCaniotController { shared })
}
