use std::default;

use crate::{
    caniot::{self, RequestData},
    controller::{DeviceActionResultTrait, DeviceActionTrait},
};

use super::actions::{DeviceAction, DeviceActionResult};

#[derive(Debug, Default)]

pub enum DeviceVerdict<A: DeviceActionTrait> {
    #[default]
    None,
    Request(RequestData),
    ActionResult(A::Result),
    ActionPendingOn(RequestData),
}

impl DeviceVerdict<DeviceAction> {
    // Converts a DeviceVerdictWrapper returned by an inner device to a DeviceVerdict<DeviceAction>
    pub fn from_inner_verdict(inner: DeviceVerdictWrapper) -> Self {
        match inner {
            DeviceVerdictWrapper::None => DeviceVerdict::None,
            DeviceVerdictWrapper::ActionResult(result) => {
                DeviceVerdict::ActionResult(DeviceActionResult::new_boxed_inner(result))
            }
            DeviceVerdictWrapper::Request(request) => DeviceVerdict::Request(request),
            DeviceVerdictWrapper::PendingActionOnRequest(request) => {
                DeviceVerdict::ActionPendingOn(request)
            }
        }
    }
}

impl<A: DeviceActionTrait> DeviceVerdict<A> {
    pub fn is_pending_action(&self) -> bool {
        matches!(self, DeviceVerdict::ActionPendingOn(_))
    }

    pub fn get_request_action_pending_on(&self) -> Option<&RequestData> {
        match self {
            DeviceVerdict::ActionPendingOn(request) => Some(request),
            _ => None,
        }
    }
}

impl<A> DeviceActionResultTrait for DeviceVerdict<A> where A: DeviceActionTrait {}

pub enum DeviceVerdictWrapper {
    None,
    ActionResult(Box<dyn DeviceActionResultTrait>),
    Request(RequestData),
    PendingActionOnRequest(RequestData),
}

impl<A> From<DeviceVerdict<A>> for DeviceVerdictWrapper
where
    A: DeviceActionTrait,
{
    fn from(verdict: DeviceVerdict<A>) -> Self {
        match verdict {
            DeviceVerdict::None => DeviceVerdictWrapper::None,
            DeviceVerdict::ActionResult(result) => DeviceVerdictWrapper::ActionResult(Box::new(
                result,
            )
                as Box<dyn DeviceActionResultTrait>),
            DeviceVerdict::Request(request) => DeviceVerdictWrapper::Request(request),
            DeviceVerdict::ActionPendingOn(request) => {
                DeviceVerdictWrapper::PendingActionOnRequest(request)
            }
        }
    }
}
