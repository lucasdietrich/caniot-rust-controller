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

#[derive(Debug, Default)]
pub struct NgCaniotController {}

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

    async fn hello_empty(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty { }))
    }
}

pub fn get_ng_caniot_controller() -> CaniotControllerServer<NgCaniotController> {
    CaniotControllerServer::new(NgCaniotController::default())
}
