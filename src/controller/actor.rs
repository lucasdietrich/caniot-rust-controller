use chrono::{DateTime, Utc};
use tokio::sync::{mpsc, oneshot};

use crate::{
    bus::CanStats,
    caniot::{self as ct},
    caniot::{self, DeviceId},
};
use serde::Serialize;

use super::{
    Controller, ControllerAPI, ControllerError, ControllerStats, DemoAction, DeviceAction,
    DeviceActionResult, DeviceActionTrait, DeviceStats, GarageDoorCommand,
};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(ControllerStats, Vec<DeviceStatsEntry>, CanStats)>,
    },
    Query {
        query: caniot::Request,
        timeout_ms: Option<u32>,
        respond_to: Option<oneshot::Sender<Result<caniot::Response, ControllerError>>>,
    },
    DeviceAction {
        did: DeviceId,
        action: DeviceAction,
        respond_to: oneshot::Sender<Result<DeviceActionResult, ControllerError>>,
    },
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    sender: mpsc::Sender<ControllerMessage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceStatsEntry {
    pub did: caniot::DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub stats: DeviceStats,
}

impl ControllerHandle {
    pub fn new(sender: mpsc::Sender<ControllerMessage>) -> Self {
        Self { sender }
    }

    /// Create a one-shot channel, embed it in a message using the provided closure, and send the
    /// message to the controller actor. Wait for the response and return it.
    async fn prepare_and_send<R>(
        &self,
        closure: impl FnOnce(oneshot::Sender<R>) -> ControllerMessage,
    ) -> R {
        let (sender, receiver) = oneshot::channel();
        let message = closure(sender);
        self.sender.send(message).await.unwrap();
        receiver.await.unwrap()
    }

    pub async fn get_stats(&self) -> (ControllerStats, Vec<DeviceStatsEntry>, CanStats) {
        self.prepare_and_send(|respond_to| ControllerMessage::GetStats { respond_to })
            .await
    }

    pub async fn device_action(
        &self,
        did: DeviceId,
        action: DeviceAction,
    ) -> Result<DeviceActionResult, ControllerError> {
        self.prepare_and_send(|respond_to| ControllerMessage::DeviceAction {
            did,
            action,
            respond_to,
        })
        .await
    }
}

#[async_trait]
impl ControllerAPI for ControllerHandle {
    async fn query(
        &self,
        frame: ct::Request,
        timeout_ms: Option<u32>,
    ) -> Result<ct::Response, ControllerError> {
        self.prepare_and_send(|sender| ControllerMessage::Query {
            query: frame,
            timeout_ms,
            respond_to: Some(sender),
        })
        .await
    }

    async fn send(&self, frame: ct::Request) -> Result<(), ControllerError> {
        self.prepare_and_send(|_sender| ControllerMessage::Query {
            query: frame,
            timeout_ms: None,
            respond_to: None,
        })
        .await
    }
}
