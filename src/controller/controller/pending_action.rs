use std::fmt::Debug;

use tokio::sync::oneshot;

use crate::{
    caniot,
    controller::{ControllerError, DeviceAction, DeviceActionResult},
};

pub struct PendingAction {
    pub action: DeviceAction,
    send_to: oneshot::Sender<Result<DeviceActionResult, ControllerError>>,

    // Response from the device which completed the action
    pub response: Option<caniot::Response>,
}

impl PendingAction {
    pub fn new(
        action: DeviceAction,
        send_to: oneshot::Sender<Result<DeviceActionResult, ControllerError>>,
    ) -> Self {
        Self {
            action,
            send_to,
            response: None,
        }
    }

    pub fn set_response(&mut self, response: caniot::Response) {
        self.response = Some(response);
    }

    pub fn send(self, result: Result<DeviceActionResult, ControllerError>) {
        let _ = self.send_to.send(result);
    }
}

impl Debug for PendingAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingAction")
            // .field("timeout_ms", &self.timeout_ms)
            // .field("issued_at", &self.issued_at)
            .finish()
    }
}
