use std::{fmt::Debug, ops::Deref};

use crate::caniot::{self, BoardClassTelemetry, Response};

use as_any::{AsAny, Downcast};

use super::{
    verdict::{ActionVerdict, ActionVerdictWrapper, Verdict},
    DeviceError, ProcessContext,
};

pub trait DeviceControllerTrait: Send + Debug + Default {
    // TODO
    // type Class: Class<'a>; ???
    type Action: ActionTrait;

    fn handle_frame(
        &mut self,
        _frame: &caniot::ResponseData,
        _as_class_blc: &Option<BoardClassTelemetry>,
        _ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        Ok(Verdict::default())
    }

    fn handle_action(
        &mut self,
        _action: &Self::Action,
        _ctx: &mut ProcessContext,
    ) -> Result<ActionVerdict<Self::Action>, DeviceError> {
        error!(
            "handle_action not implemented for device controller \"{}\"",
            self.get_infos().name.unwrap_or_default()
        );
        Err(DeviceError::NotImplemented)
    }

    // Building an action result shouldn't alter the device state (i.e. &self only)
    fn handle_action_result(
        &self,
        _delayed_action: &Self::Action,
        _completed_by: &Option<Response>,
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        error!(
            "handle_action_result not implemented for device controller \"{}\"",
            self.get_infos().name.unwrap_or_default()
        );
        Err(DeviceError::NotImplemented)
    }

    fn process(&mut self, _ctx: &mut ProcessContext) -> Result<Verdict, DeviceError> {
        Ok(Verdict::default())
    }

    fn get_infos(&self) -> DeviceControllerInfos;
}

#[derive(Debug, Default)]
pub struct DeviceControllerInfos {
    pub name: Option<String>,
}

impl DeviceControllerInfos {
    pub fn new(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
        }
    }
}

/// This trait is used to wrap a DeviceTrait into a DeviceWrapperTrait and make it object safe
/// so that we can make a list of devices with different types.
pub trait DeviceControllerWrapperTrait: Send + Debug {
    fn wrapper_handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
        as_class_blc: &Option<BoardClassTelemetry>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError>;

    // Check if the action type can be handled by this device
    fn wrapper_can_handle_action(&self, action: &dyn ActionWrapperTrait) -> bool;

    fn wrapper_handle_action(
        &mut self,
        action: &Box<dyn ActionWrapperTrait>,
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdictWrapper, DeviceError>;

    fn wrapper_handle_delayed_action_result(
        &self,
        _delayed_action: &Box<dyn ActionWrapperTrait>,
        completed_by: &Option<caniot::Response>,
    ) -> Result<Box<dyn ActionResultTrait>, DeviceError> {
        Err(DeviceError::NotImplemented)
    }

    fn wrapper_process(&mut self, ctx: &mut ProcessContext) -> Result<Verdict, DeviceError>;

    fn wrapper_get_infos(&self) -> DeviceControllerInfos;
}

/// Automatically implement DeviceWrapperTrait for any DeviceTrait
impl<T: DeviceControllerTrait> DeviceControllerWrapperTrait for T
where
    <T as DeviceControllerTrait>::Action: 'static,
{
    fn wrapper_can_handle_action(&self, action: &dyn ActionWrapperTrait) -> bool {
        action.is::<T::Action>()
    }

    fn wrapper_handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
        as_class_blc: &Option<BoardClassTelemetry>,
        ctx: &mut ProcessContext,
    ) -> Result<Verdict, DeviceError> {
        self.handle_frame(frame, as_class_blc, ctx)
    }

    fn wrapper_handle_action(
        &mut self,
        action: &Box<dyn ActionWrapperTrait>,
        ctx: &mut ProcessContext,
    ) -> Result<ActionVerdictWrapper, DeviceError> {
        match action.deref().downcast_ref::<T::Action>() {
            Some(action) => self
                .handle_action(action, ctx)
                .map(ActionVerdictWrapper::from),
            None => Err(DeviceError::UnsupportedAction),
        }
    }

    fn wrapper_handle_delayed_action_result(
        &self,
        delayed_action: &Box<dyn ActionWrapperTrait>,
        completed_by: &Option<Response>,
    ) -> Result<Box<dyn ActionResultTrait>, DeviceError> {
        match delayed_action.deref().downcast_ref::<T::Action>() {
            Some(delayed_action) => self
                .handle_action_result(delayed_action, completed_by)
                .map(|result| Box::new(result) as Box<dyn ActionResultTrait>),
            None => Err(DeviceError::UnsupportedAction),
        }
    }

    fn wrapper_process(&mut self, ctx: &mut ProcessContext) -> Result<Verdict, DeviceError> {
        self.process(ctx)
    }

    fn wrapper_get_infos(&self) -> DeviceControllerInfos {
        self.get_infos()
    }
}

pub trait ActionTrait: AsAny + Send + Debug {
    type Result: ActionResultTrait; // TODO Check if Clone trait can be added here
}

pub trait ActionResultTrait: AsAny + Send {}

pub trait ActionWrapperTrait: AsAny + Send + Debug {}

impl<T> ActionWrapperTrait for T where T: ActionTrait + Debug {}
