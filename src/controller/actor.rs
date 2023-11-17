use tokio::sync::{mpsc, oneshot};

use crate::{
    can::CanStats,
    caniot::{self, build_telemetry_request},
};
use serde::Serialize;

use super::{Controller, ControllerError, ControllerStats, DeviceStats};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(ControllerStats, Vec<DeviceStatsEntry>, CanStats)>,
    },
    Query {
        query: caniot::Request,
        timeout_ms: u32,
        respond_to: oneshot::Sender<Result<caniot::Response, ControllerError>>,
    }
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    pub sender: mpsc::Sender<ControllerMessage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceStatsEntry {
    pub device_id_did: u8,
    pub device_id: caniot::DeviceId,
    pub stats: DeviceStats,
}

impl ControllerHandle {
    pub async fn get_stats(
        &self,
    ) -> Result<(ControllerStats, Vec<DeviceStatsEntry>, CanStats), ()> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::GetStats { respond_to };
        self.sender.send(msg).await.unwrap();
        recv.await.map_err(|_| ())
    }

    pub async fn query(
        &self,
        frame: caniot::Request,
        timeout_ms: u32,
    ) -> Result<caniot::Response, ControllerError> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::Query {
            query: frame,
            timeout_ms,
            respond_to,
        };
        self.sender.send(msg).await.unwrap();
        let response = recv.await.unwrap();
        response
    }

    pub async fn query_telemetry(
        &self,
        device_id: caniot::DeviceId,
        endpoint: caniot::Endpoint,
        timeout_ms: u32,
    ) -> Result<caniot::Response, ControllerError> {
        self.query(build_telemetry_request(device_id, endpoint), timeout_ms)
            .await
    }
}

pub async fn handle_message(controller: &mut Controller, message: ControllerMessage) {
    match message {
        ControllerMessage::GetStats { respond_to } => {
            let _ = respond_to.send((
                controller.stats,
                controller.get_devices_stats(),
                controller.iface.stats,
            ));
        }
        ControllerMessage::Query {
            query,
            timeout_ms,
            respond_to,
        } => {
            controller.query(query, timeout_ms, respond_to).await;
        }
    }
}
