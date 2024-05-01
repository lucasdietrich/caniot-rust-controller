use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use tokio::sync::oneshot;

use crate::{
    caniot,
    controller::{ControllerError, DeviceAction, DeviceActionResult},
};

const DEFAULT_TIMEOUT_MS: usize = 1000;

pub struct PendingAction {
    pub action: DeviceAction,
    pub issued_at: Instant,
    pub timeout_ms: usize,
    send_to: oneshot::Sender<Result<DeviceActionResult, ControllerError>>,
    pq_receiver: oneshot::Receiver<Result<caniot::Response, ControllerError>>,
}

impl PendingAction {
    pub fn new(
        action: DeviceAction,
        send_to: oneshot::Sender<Result<DeviceActionResult, ControllerError>>,
        timeout_ms: Option<usize>,
        pq_receiver: oneshot::Receiver<Result<caniot::Response, ControllerError>>,
    ) -> Self {
        Self {
            action,
            timeout_ms: timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS),
            issued_at: Instant::now(),
            send_to,
            pq_receiver,
        }
    }

    pub fn answer(self, result: Result<DeviceActionResult, ControllerError>) {
        let _ = self.send_to.send(result);
    }

    pub fn is_expired(&self) -> bool {
        self.issued_at.elapsed().as_millis() as usize >= self.timeout_ms
    }

    // pub async fn ready(&self) -> bool {
    //     let z = self.pq_receiver.await;
    // }
}

impl Debug for PendingAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingAction")
            .field("timeout_ms", &self.timeout_ms)
            .field("issued_at", &self.issued_at)
            .finish()
    }
}
