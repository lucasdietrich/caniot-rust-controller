use std::{
    any::{Any, TypeId},
    fmt::Debug,
    ops::Deref,
};

use crate::caniot;

use as_any::{AsAny, Downcast};

use super::{
    verdict::{DeviceVerdict, DeviceVerdictWrapper},
    DeviceError, DeviceEvent, DeviceProcessContext,
};

pub trait DeviceTrait: Send + Debug {
    type Action: DeviceActionTrait;

    fn handle_frame(
        &mut self,
        _frame: &caniot::ResponseData,
        _action: Option<&Self::Action>, // Action to which the response frame is related if any
        _ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdict<Self::Action>, DeviceError> {
        Ok(DeviceVerdict::default())
    }

    fn handle_action(
        &mut self,
        _action: &Self::Action,
        _ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdict<Self::Action>, DeviceError> {
        Err(DeviceError::NotImplemented)
    }

    // // Building an action result shouldn't alter the device state (i.e. &self only)
    // fn handle_delayed_action_result(
    //     &self,
    //     _delayed_action: &Self::Action,
    // ) -> Result<<Self::Action as DeviceActionTrait>::Result, DeviceError> {
    //     Err(DeviceError::NotImplemented)
    // }

    fn process(
        &mut self,
        _ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdict<Self::Action>, DeviceError> {
        Ok(DeviceVerdict::default())
    }

    fn handle_event(
        &mut self,
        event: &DeviceEvent<Self::Action>,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdict<Self::Action>, DeviceError> {
        match event {
            DeviceEvent::Process => self.process(ctx),
            DeviceEvent::Action(action) => self.handle_action(action, ctx),
            DeviceEvent::Frame(frame) => self.handle_frame(frame, None, ctx),
            DeviceEvent::FrameForAction(frame, action) => {
                self.handle_frame(frame, Some(action), ctx)
            }
        }
    }
}

/// This trait is used to wrap a DeviceTrait into a DeviceWrapperTrait and make it object safe
/// so that we can make a list of devices with different types.
pub trait DeviceWrapperTrait: Send + Debug {
    fn wrapper_handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
        action: Option<&Box<dyn DeviceActionWrapperTrait>>,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdictWrapper, DeviceError>;

    // Check if the action type can be handled by this device
    fn wrapper_can_handle_action(&self, action: &dyn DeviceActionWrapperTrait) -> bool;

    fn wrapper_handle_action(
        &mut self,
        action: &Box<dyn DeviceActionWrapperTrait>,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdictWrapper, DeviceError>;

    // fn wrapper_handle_delayed_action_result(
    //     &self,
    //     delayed_action: &Box<dyn DeviceActionWrapperTrait>,
    //     ctx: &mut DeviceProcessContext,
    // ) -> Result<Box<dyn DeviceActionResultTrait>, DeviceError> {
    //     Err(DeviceError::NotImplemented)
    // }

    fn wrapper_process(
        &mut self,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdictWrapper, DeviceError>;
}

/// Automatically implement DeviceWrapperTrait for any DeviceTrait
impl<T: DeviceTrait> DeviceWrapperTrait for T
where
    <T as DeviceTrait>::Action: 'static,
{
    fn wrapper_can_handle_action(&self, action: &dyn DeviceActionWrapperTrait) -> bool {
        action.is::<T::Action>()
    }

    fn wrapper_handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
        action: Option<&Box<dyn DeviceActionWrapperTrait>>,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdictWrapper, DeviceError> {
        let action = action
            .map(|action| match action.deref().downcast_ref::<T::Action>() {
                Some(action) => Ok(action),
                None => Err(DeviceError::UnsupportedAction),
            })
            .transpose()?;

        self.handle_frame(frame, action, ctx)
            .map(DeviceVerdictWrapper::from)
    }

    fn wrapper_handle_action(
        &mut self,
        action: &Box<dyn DeviceActionWrapperTrait>,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdictWrapper, DeviceError> {
        match action.deref().downcast_ref::<T::Action>() {
            Some(action) => self
                .handle_action(action, ctx)
                .map(DeviceVerdictWrapper::from),
            None => Err(DeviceError::UnsupportedAction),
        }
    }

    // fn wrapper_handle_delayed_action_result(
    //     &self,
    //     delayed_action: &Box<dyn DeviceActionWrapperTrait>,
    //     ctx: &mut DeviceProcessContext,
    // ) -> Result<Box<dyn DeviceActionResultTrait>, DeviceError> {
    //     match delayed_action.deref().downcast_ref::<T::Action>() {
    //         Some(delayed_action) => self
    //             .handle_delayed_action_result(delayed_action)
    //             .map(|result| Box::new(result) as Box<dyn DeviceActionResultTrait>),
    //         None => Err(DeviceError::UnsupportedAction),
    //     }
    // }

    fn wrapper_process(
        &mut self,
        ctx: &mut DeviceProcessContext,
    ) -> Result<DeviceVerdictWrapper, DeviceError> {
        self.process(ctx).map(DeviceVerdictWrapper::from)
    }
}

pub trait DeviceActionTrait: AsAny + Send {
    type Result: DeviceActionResultTrait; // TODO Check if Clone trait can be added here
}

pub trait DeviceActionResultTrait: AsAny + Send {}

pub trait DeviceActionWrapperTrait: AsAny + Send {}

impl<T> DeviceActionWrapperTrait for T where T: DeviceActionTrait {}
