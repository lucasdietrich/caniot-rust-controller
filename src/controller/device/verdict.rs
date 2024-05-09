

use crate::{
    caniot::{RequestData},
    controller::{ActionResultTrait, ActionTrait},
};

use super::actions::{DeviceAction, DeviceActionResult};

#[derive(Debug, Default)]
pub enum Verdict {
    #[default]
    None,
    Request(RequestData),
}

#[derive(Debug)]
pub enum ActionVerdict<A: ActionTrait> {
    ActionResult(A::Result),
    ActionPendingOn(RequestData),
}

impl ActionVerdict<DeviceAction> {
    // Converts a DeviceVerdictWrapper returned by an inner device to a DeviceVerdict<DeviceAction>
    pub fn from_inner_verdict(inner: ActionVerdictWrapper) -> Self {
        match inner {
            ActionVerdictWrapper::ActionResult(result) => {
                ActionVerdict::ActionResult(DeviceActionResult::new_boxed_inner(result))
            }
            ActionVerdictWrapper::PendingActionOnRequest(request) => {
                ActionVerdict::ActionPendingOn(request)
            }
        }
    }
}

impl<A: ActionTrait> ActionVerdict<A> {
    pub fn is_pending_action(&self) -> bool {
        matches!(self, ActionVerdict::ActionPendingOn(_))
    }

    pub fn get_request_action_pending_on(&self) -> Option<&RequestData> {
        match self {
            ActionVerdict::ActionPendingOn(request) => Some(request),
            _ => None,
        }
    }
}

impl<A> ActionResultTrait for ActionVerdict<A> where A: ActionTrait {}

pub enum ActionVerdictWrapper {
    ActionResult(Box<dyn ActionResultTrait>),
    PendingActionOnRequest(RequestData),
}

impl<A> From<ActionVerdict<A>> for ActionVerdictWrapper
where
    A: ActionTrait,
{
    fn from(verdict: ActionVerdict<A>) -> Self {
        match verdict {
            ActionVerdict::ActionResult(result) => {
                ActionVerdictWrapper::ActionResult(Box::new(result) as Box<dyn ActionResultTrait>)
            }
            ActionVerdict::ActionPendingOn(request) => {
                ActionVerdictWrapper::PendingActionOnRequest(request)
            }
        }
    }
}
