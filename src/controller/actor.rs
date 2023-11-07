use tokio::sync::{mpsc, oneshot};

use crate::{
    can::CanStats,
    caniot::{self, build_telemetry_request},
};

use super::{CaniotStats, Controller, ControllerError};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(CaniotStats, CanStats)>,
    },
    QueryFrame {
        query: caniot::Request,
        timeout_ms: u32,
        respond_to: oneshot::Sender<Result<caniot::Response, ControllerError>>,
    },
    Query {
        // query: caniot::Request,
        // timeout_ms: u32,
        respond_to: oneshot::Sender<()>,
    },
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    pub sender: mpsc::Sender<ControllerMessage>,
}

impl ControllerHandle {
    pub async fn query(&self) -> Result<(), ()> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::Query { respond_to };
        self.sender.send(msg).await.unwrap();
        recv.await.map_err(|_| ())
    }

    pub async fn get_stats(&self) -> Result<(CaniotStats, CanStats), ()> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::GetStats { respond_to };
        self.sender.send(msg).await.unwrap();
        recv.await.map_err(|_| ())
    }

    pub async fn query_frame(
        &self,
        frame: caniot::Request,
        timeout_ms: u32,
    ) -> Result<caniot::Response, ControllerError> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::QueryFrame {
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
        self.query_frame(build_telemetry_request(device_id, endpoint), timeout_ms)
            .await
    }
}

pub async fn handle_message(controller: &mut Controller, message: ControllerMessage) {
    match message {
        ControllerMessage::GetStats { respond_to } => {
            let _ = respond_to.send((controller.stats, controller.iface.stats));
        }
        ControllerMessage::QueryFrame {
            query,
            timeout_ms,
            respond_to,
        } => {
            controller.query(query, timeout_ms, respond_to).await;
        }
        ControllerMessage::Query { respond_to } => {
            let _ = respond_to.send(());
        }
    }
}
