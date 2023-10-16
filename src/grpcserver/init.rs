use log::info;
use tonic::{
    transport::{Error as GrpcError, Server},
    Code, Request, Response, Status,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::shared::SharedHandle;
use super::ng::get_ng_caniot_controller;

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

pub async fn grpc_server(shared: SharedHandle) -> Result<(), GrpcServerInitError> {
    let addr = &shared.config.grpc.listen;
    let addr = addr.parse().expect("gRPC: Could not parse listen address");
    let ng_controller = get_ng_caniot_controller();

    let mut rx: tokio::sync::broadcast::Receiver<()> = shared.notify_shutdown.subscribe();
    let shutdown_future = async move {
        let _ = rx.recv().await;
        info!("gRPC server shutting down...");
    };

    info!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(ng_controller)
        .serve_with_shutdown(addr, shutdown_future)
        .await?;

    Ok(())
}
