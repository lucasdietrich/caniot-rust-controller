use std::time::{Duration, Instant};

use super::{ControllerError, PendingAction};
use crate::{caniot, utils::expirable::ExpirableTrait};
use tokio::sync::oneshot;

/// Initiator of a pending query, it represents the entity that is waiting for the query to be answered
#[derive(Debug)]
pub enum PendingQueryTenant {
    // channel to reply to when query is answered
    Query(oneshot::Sender<Result<caniot::Response, ControllerError>>),

    // channel to reply when responses are received
    // Broadcast(mpsc::Sender<Result<caniot::Response, ControllerError>>),

    // The pending action the query is associated with
    Action(PendingAction),
}

impl PendingQueryTenant {
    pub fn end_with_error(self, error: ControllerError) -> Option<PendingQueryTenant> {
        match self {
            Self::Query(sender) => {
                let _ = sender.send(Err(error)); // Do not panic if receiver is dropped
                None
            }
            Self::Action(pending_action) => {
                pending_action.send(Err(error));
                None
            }
        }
    }

    pub fn end_with_frame(self, frame: caniot::Response) -> Option<PendingQueryTenant> {
        match self {
            Self::Query(sender) => {
                let _ = sender.send(Ok(frame)); // Do not panic if receiver is dropped
                None
            }
            Self::Action(mut pending_action) => {
                pending_action.set_response(frame);
                Some(PendingQueryTenant::Action(pending_action))
            }
        }
    }
}

#[derive(Debug)]
pub struct PendingQuery {
    pub tenant: PendingQueryTenant,

    // query pending
    pub query: caniot::Request,

    // timeout in milliseconds
    pub timeout_ms: u32,

    // time when query was sent
    sent_at: std::time::Instant,
}

impl PendingQuery {
    pub fn new(query: caniot::Request, timeout_ms: u32, tenant: PendingQueryTenant) -> Self {
        Self {
            query,
            timeout_ms,
            sent_at: std::time::Instant::now(),
            tenant,
        }
    }

    pub fn end_with_frame(self, frame: caniot::Response) -> Option<PendingQueryTenant> {
        self.tenant.end_with_frame(frame)
    }

    pub fn end_with_error(self, error: ControllerError) -> Option<PendingQueryTenant> {
        self.tenant.end_with_error(error)
    }

    #[allow(dead_code)]
    pub fn end(
        self,
        response: Result<caniot::Response, ControllerError>,
    ) -> Option<PendingQueryTenant> {
        match response {
            Ok(frame) => self.end_with_frame(frame),
            Err(error) => self.end_with_error(error),
        }
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

impl ExpirableTrait<Duration> for PendingQuery {
    const ZERO: Duration = Duration::ZERO;
    type Instant = Instant;

    fn ttl(&self, now: &Instant) -> Option<Duration> {
        let timeout_instant = self.get_timeout_instant();
        if *now < timeout_instant {
            Some(timeout_instant - *now)
        } else {
            None
        }
    }
}
