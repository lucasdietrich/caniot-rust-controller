use tokio::sync::{mpsc, oneshot};

use crate::{can::CanStats, caniot};

use super::{CaniotStats, Controller};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(CaniotStats, CanStats)>,
    },
    QueryTelemetry {
        device_id: caniot::DeviceId,
        endpoint: caniot::Endpoint,
        timeout_ms: u32,
        respond_to: oneshot::Sender<Option<caniot::Response>>,
    },
    Query {
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

    pub async fn query_telemetry(
        &self,
        device_id: caniot::DeviceId,
        endpoint: caniot::Endpoint,
        timeout_ms: u32,
    ) -> Result<Option<caniot::Response>, ()> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::QueryTelemetry {
            device_id,
            endpoint,
            timeout_ms,
            respond_to,
        };
        self.sender.send(msg).await.unwrap();
        let response = recv.await.unwrap();
        Ok(response)
    }
}

pub async fn handle_message(controller: &mut Controller, message: ControllerMessage) {
    match message {
        ControllerMessage::GetStats { respond_to } => {
            let _ = respond_to.send((controller.stats, controller.iface.stats));
        }
        ControllerMessage::QueryTelemetry {
            device_id,
            endpoint,
            timeout_ms,
            respond_to,
        } => {
            let request = caniot::build_telemetry_request(device_id, endpoint);
            let result = controller
                .query(
                    request,
                    timeout_ms,
                    Box::new(move |result| {
                        let _ = respond_to.send(result.ok());
                    }),
                )
                .await;

            // Return None if the query failed
            // TODO replace this Result<>
            // if result.is_err() {
            //     respond_to.send(None);
            // }
        }
        ControllerMessage::Query { respond_to } => {
            let _ = respond_to.send(());
        }
    }
}
