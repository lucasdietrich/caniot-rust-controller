use std::time::{Duration, Instant};

use super::ControllerError;
use crate::caniot;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct PendingQuery {
    // query pending
    pub query: caniot::Request,

    // closure to call when response is received
    sender: oneshot::Sender<Result<caniot::Response, ControllerError>>,

    // timeout in milliseconds
    pub timeout_ms: u32,

    // time when query was sent
    sent_at: std::time::Instant,
}

impl PendingQuery {
    pub fn new(
        query: caniot::Request,
        sender: oneshot::Sender<Result<caniot::Response, ControllerError>>,
        timeout_ms: u32,
    ) -> Self {
        Self {
            query,
            sender,
            timeout_ms,
            sent_at: std::time::Instant::now(),
        }
    }

    /// Answer the pending query with a response
    pub fn reply(self, response: Result<caniot::Response, ControllerError>) {
        let _ = self.sender.send(response); // Do not panic if receiver is dropped
    }

    /// Answer the pending query with a response
    pub fn reply_with_frame(self, response: caniot::Response) {
        self.reply(Ok(response))
    }

    /// Check whether the response matches the query
    pub fn match_response(&self, response: &caniot::Response) -> bool {
        caniot::is_response_to(&self.query, response).is_response()
    }

    /// Get the instant when the query will timeout
    pub fn get_timeout_instant(&self) -> std::time::Instant {
        self.sent_at + Duration::from_millis(self.timeout_ms as u64)
    }

    /// Check whether the query has timed out
    pub fn has_timed_out(&self, now: &Instant) -> bool {
        now.duration_since(self.sent_at) >= Duration::from_millis(self.timeout_ms as u64)
    }
}
