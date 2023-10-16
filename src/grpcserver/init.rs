use log::info;
use tonic::{
    transport::{Error as GrpcError, Server},
    Code, Request, Response, Status,
};

use model::can_controller_server::{CanController, CanControllerServer};
use model::*;

pub mod model {
    tonic::include_proto!("cancontroller.ipc");
}

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::shared::SharedHandle;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrpcConfig {
    pub listen: String,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            listen: "[::]:50051".to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum GrpcServerInitError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("gRPC Error: {0}")]
    GrpcError(#[from] GrpcError),
}

#[derive(Debug, Default)]
struct MyCanController {}

#[tonic::async_trait]
impl CanController for MyCanController {
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

    async fn get_device(&self, request: Request<DeviceId>) -> Result<Response<Device>, Status> {
        println!("Got a request: {:?}", request);

        let device = Device {
            deviceid: request.into_inner().into(),
            name: "test".to_string(),
            version: 0,
        };

        Ok(Response::new(device))
    }
}

pub async fn grpc_server(shared: SharedHandle) -> Result<(), GrpcServerInitError> {
    let addr = &shared.config.grpc.listen;
    let addr = addr.parse().expect("gRPC: Could not parse listen address");
    let controller = MyCanController::default();

    let mut rx: tokio::sync::broadcast::Receiver<()> = shared.notify_shutdown.subscribe();
    let shutdown_future = async move {
        let _ = rx.recv().await;
        info!("gRPC server shutting down...");
    };

    info!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(CanControllerServer::new(controller))
        .serve_with_shutdown(addr, shutdown_future)
        .await?;

    Ok(())
}
