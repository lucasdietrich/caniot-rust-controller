use std::default;

use crate::{
    caniot::{self, RequestData},
    controller::{DeviceActionResultTrait, DeviceActionTrait},
};

use super::actions::{DeviceAction, DeviceActionResult};

#[derive(Debug, Default)]
pub enum DeviceVerdict {
    #[default]
    None,
    Request(RequestData),
}

#[derive(Debug)]
pub enum DeviceActionVerdict<A: DeviceActionTrait> {
    ActionResult(A::Result),
    ActionPendingOn(RequestData),
}

impl DeviceActionVerdict<DeviceAction> {
    // Converts a DeviceVerdictWrapper returned by an inner device to a DeviceVerdict<DeviceAction>
    pub fn from_inner_verdict(inner: DeviceActionVerdictWrapper) -> Self {
        match inner {
            DeviceActionVerdictWrapper::ActionResult(result) => {
                DeviceActionVerdict::ActionResult(DeviceActionResult::new_boxed_inner(result))
            }
            DeviceActionVerdictWrapper::PendingActionOnRequest(request) => {
                DeviceActionVerdict::ActionPendingOn(request)
            }
        }
    }
}

impl<A: DeviceActionTrait> DeviceActionVerdict<A> {
    pub fn is_pending_action(&self) -> bool {
        matches!(self, DeviceActionVerdict::ActionPendingOn(_))
    }

    pub fn get_request_action_pending_on(&self) -> Option<&RequestData> {
        match self {
            DeviceActionVerdict::ActionPendingOn(request) => Some(request),
            _ => None,
        }
    }
}

impl<A> DeviceActionResultTrait for DeviceActionVerdict<A> where A: DeviceActionTrait {}

pub enum DeviceActionVerdictWrapper {
    ActionResult(Box<dyn DeviceActionResultTrait>),
    PendingActionOnRequest(RequestData),
}

impl<A> From<DeviceActionVerdict<A>> for DeviceActionVerdictWrapper
where
    A: DeviceActionTrait,
{
    fn from(verdict: DeviceActionVerdict<A>) -> Self {
        match verdict {
            DeviceActionVerdict::ActionResult(result) => DeviceActionVerdictWrapper::ActionResult(
                Box::new(result) as Box<dyn DeviceActionResultTrait>,
            ),
            DeviceActionVerdict::ActionPendingOn(request) => {
                DeviceActionVerdictWrapper::PendingActionOnRequest(request)
            }
        }
    }
}
