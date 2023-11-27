use tokio::{
    net::unix::pipe::Receiver,
    sync::{mpsc, oneshot},
};

use crate::{
    can::CanStats,
    caniot::{self, build_telemetry_request, DeviceId, Endpoint, Response},
};
use serde::Serialize;

use super::{Controller, ControllerError, ControllerStats, DeviceStats, GarageHandle};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(ControllerStats, Vec<DeviceStatsEntry>, CanStats)>,
    },
    Query {
        query: caniot::Request,
        timeout_ms: u32,
        respond_to: oneshot::Sender<Result<caniot::Response, ControllerError>>,
    },
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

// trait ControllerAPI {
// async fn query(&self, frame: caniot::Request,
//     timeout_ms: u32,
// ) -> Result<caniot::Response, ControllerError>;

// pub async fn query_telemetry(
//     &self,
//     device_id: caniot::DeviceId,
//     endpoint: caniot::Endpoint,
//     timeout_ms: u32,
// ) -> Result<caniot::Response, ControllerError> {
//     self.query(build_telemetry_request(device_id, endpoint), timeout_ms)
//         .await
// }
// }

impl ControllerHandle {
    async fn execute<R>(&self, closure: impl FnOnce(oneshot::Sender<R>) -> ControllerMessage) -> R {
        let (sender, receiver) = oneshot::channel();
        let message = closure(sender);
        self.sender.send(message).await.unwrap();
        receiver.await.unwrap()
    }

    pub async fn get_stats(&self) -> (ControllerStats, Vec<DeviceStatsEntry>, CanStats) {
        self.execute(|respond_to| ControllerMessage::GetStats { respond_to })
            .await
    }

    pub async fn query(
        &self,
        frame: caniot::Request,
        timeout_ms: u32,
    ) -> Result<caniot::Response, ControllerError> {
        self.execute(|sender| ControllerMessage::Query {
            query: frame,
            timeout_ms,
            respond_to: sender,
        })
        .await
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

    pub fn get_device(&self, did: DeviceId) -> DeviceHandle {
        DeviceHandle::new(did, self)
    }

    pub fn get_garage_handle(&self) -> GarageHandle {
        GarageHandle::new(self)
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

#[derive(Debug, Clone)]
pub struct DeviceHandle<'a> {
    did: DeviceId,
    controller_handle: &'a ControllerHandle,
}

impl<'a> DeviceHandle<'a> {
    fn new(did: DeviceId, controller_handle: &'a ControllerHandle) -> DeviceHandle {
        DeviceHandle {
            did,
            controller_handle,
        }
    }

    pub async fn request_telemetry(
        &self,
        endpoint: Endpoint,
        timeout_ms: u32,
    ) -> Result<Response, ControllerError> {
        self.controller_handle
            .query_telemetry(self.did, endpoint, timeout_ms)
            .await
    }
}
