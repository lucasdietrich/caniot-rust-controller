use log::info;
use tonic::transport::{Error as GrpcError, Server};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::ng::get_ng_internal_server;
use crate::{
    grpcserver::{
        legacy::get_legacy_caniot_controller,
        ng::{
            get_ng_alarms_server, get_ng_controller_server, get_ng_devices_server,
            get_ng_emulation_server, get_ng_garage_server, get_ng_heaters_server,
        },
    },
    shared::SharedHandle,
};

#[cfg(feature = "grpc-can-iface-server")]
use crate::grpcserver::ng::get_ng_can_iface_server;

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

    let ng_controller = get_ng_controller_server(shared.clone());
    let ng_internal = get_ng_internal_server(shared.clone());
    let ng_devices = get_ng_devices_server(shared.clone());
    let ng_heaters = get_ng_heaters_server(shared.clone());
    let ng_alarms = get_ng_alarms_server(shared.clone());
    let ng_garage = get_ng_garage_server(shared.clone());
    let ng_emu = get_ng_emulation_server(shared.clone());
    let legacy_controller = get_legacy_caniot_controller(shared.clone());
    #[cfg(feature = "grpc-can-iface-server")]
    let ng_can_iface = get_ng_can_iface_server(shared.clone());

    let mut rx: tokio::sync::broadcast::Receiver<()> = shared.notify_shutdown.subscribe();
    let shutdown_future = async move {
        let _ = rx.recv().await;
        info!("gRPC server shutting down...");
    };

    info!("gRPC server listening on {}", addr);

    let builder = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(ng_controller))
        .add_service(tonic_web::enable(ng_internal))
        .add_service(tonic_web::enable(ng_devices))
        .add_service(tonic_web::enable(ng_heaters))
        .add_service(tonic_web::enable(ng_garage))
        .add_service(tonic_web::enable(ng_alarms))
        .add_service(tonic_web::enable(ng_emu))
        .add_service(tonic_web::enable(legacy_controller));

    #[cfg(feature = "grpc-can-iface-server")]
    let builder = { builder.add_service(tonic_web::enable(ng_can_iface)) };

    // Start
    builder.serve_with_shutdown(addr, shutdown_future).await?;

    info!("gRPC server stopped");

    Ok(())
}
