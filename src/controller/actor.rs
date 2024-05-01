use std::ops::Deref;

use as_any::{AsAny, Downcast};
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
    DeviceActionResult, DeviceActionResultTrait, DeviceActionTrait, DeviceStats, GarageDoorCommand,
};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(ControllerStats, Vec<DeviceStatsEntry>, CanStats)>,
    },
    Query {
        query: caniot::Request,
        timeout_ms: Option<u32>,

        // If Some, the controller will respond to the sender with the result of the query.
        // If None, the controller will send the query and not wait for a response.
        respond_to: Option<oneshot::Sender<Result<caniot::Response, ControllerError>>>,
    },
    DeviceAction {
        did: Option<DeviceId>,
        action: DeviceAction,
        respond_to:
            oneshot::Sender<Result<<DeviceAction as DeviceActionTrait>::Result, ControllerError>>,
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

    /// Query a controller message
    ///
    /// Create a one-shot channel, embed it in a message using the provided closure, and send the
    /// message to the controller actor. Wait for the response and return it.
    async fn query<R>(
        &self,
        build_message_closure: impl FnOnce(oneshot::Sender<R>) -> ControllerMessage,
    ) -> R {
        let (sender, receiver) = oneshot::channel();
        let message = build_message_closure(sender);
        self.sender.send(message).await.unwrap();
        receiver.await.expect("Sender dropped before response")
    }

    pub async fn get_stats(&self) -> (ControllerStats, Vec<DeviceStatsEntry>, CanStats) {
        self.query(|respond_to| ControllerMessage::GetStats { respond_to })
            .await
    }

    // Send a generic device action to the controller of the device.
    // Some actions are unique to a specific device, so the device id is optional.
    // For generic actions, the device id is required.
    pub async fn device_action(
        &self,
        did: Option<DeviceId>,
        action: DeviceAction,
    ) -> Result<DeviceActionResult, ControllerError> {
        self.query(|respond_to| ControllerMessage::DeviceAction {
            did,
            action,
            respond_to,
        })
        .await
    }

    // Send a specific (typed) device action to the controller of the device.
    pub async fn device_action_inner<A: DeviceActionTrait>(
        &self,
        did: Option<DeviceId>,
        action: A,
    ) -> Result<A::Result, ControllerError>
    // # IMPORTANT NOTE: TODO
    // The A::Result type which is returned by the action must implement the Clone trait.
    // check if A::Result can be constrained to implement Clone
    // for sure DeviceActionResultTrait cannot implement Clone as it would make it not object-safe
    //
    // Evaluate DeviceActionResultTrait { type Result: DeviceActionResultTrait + Clone }
    where
        A::Result: Clone,
    {
        let action = DeviceAction::new_inner(action);
        let result = self.device_action(did, action).await?;
        match result {
            DeviceActionResult::Inner(inner) => match inner.deref().downcast_ref::<A::Result>() {
                Some(result) => Ok(result.clone()),
                None => panic!("Unexpected DeviceActionResult inner variant"),
            },
            _ => panic!("Unexpected DeviceActionResult variant"),
        }
        // Err(ControllerError::NotImplemented)
    }
}

#[async_trait]
impl ControllerAPI for ControllerHandle {
    async fn query(
        &self,
        frame: ct::Request,
        timeout_ms: Option<u32>,
    ) -> Result<ct::Response, ControllerError> {
        self.query(|sender| ControllerMessage::Query {
            query: frame,
            timeout_ms,
            respond_to: Some(sender),
        })
        .await
    }

    async fn send(&self, frame: ct::Request) -> Result<(), ControllerError> {
        self.query(|_sender| ControllerMessage::Query {
            query: frame,
            timeout_ms: None,
            respond_to: None,
        })
        .await
    }
}
