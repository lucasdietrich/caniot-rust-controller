use std::{
    any::{Any, TypeId},
    fmt::Debug,
    ops::Deref,
};

use crate::caniot;

use as_any::{AsAny, Downcast};

use super::{DeviceError, DeviceEvent, DeviceProcessOutput, DeviceProcessOutputWrapper};

pub trait DeviceTrait: Send + Debug {
    type Action: DeviceActionTrait;

    fn handle_frame(
        &mut self,
        _frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        Ok(DeviceProcessOutput::default())
    }

    fn handle_action(
        &mut self,
        _action: &Self::Action,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        Err(DeviceError::NotImplemented)
    }

    fn process(&mut self) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        Ok(DeviceProcessOutput::default())
    }

    fn handle_event(
        &mut self,
        event: &DeviceEvent<Self::Action>,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError> {
        match event {
            DeviceEvent::Process => self.process(),
            DeviceEvent::Action(action) => self.handle_action(action),
            DeviceEvent::Frame(frame) => self.handle_frame(frame),
        }
    }
}

/// This trait is used to wrap a DeviceTrait into a DeviceWrapperTrait and make it object safe
/// so that we can make a list of devices with different types.
pub trait DeviceWrapperTrait: Send + Debug {
    fn wrapper_handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutputWrapper, DeviceError>;

    // Check if the action type can be handled by this device
    fn wrapper_can_handle_action(&self, action: &dyn DeviceActionWrapperTrait) -> bool;

    fn wrapper_handle_action(
        &mut self,
        action: &Box<dyn DeviceActionWrapperTrait>,
    ) -> Result<DeviceProcessOutputWrapper, DeviceError>;

    fn wrapper_process(&mut self) -> Result<DeviceProcessOutputWrapper, DeviceError>;
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
    ) -> Result<DeviceProcessOutputWrapper, DeviceError> {
        self.handle_frame(frame)
            .map(DeviceProcessOutputWrapper::from)
    }

    fn wrapper_handle_action(
        &mut self,
        action: &Box<dyn DeviceActionWrapperTrait>,
    ) -> Result<DeviceProcessOutputWrapper, DeviceError> {
        match action.deref().downcast_ref::<T::Action>() {
            Some(action) => self
                .handle_action(action)
                .map(DeviceProcessOutputWrapper::from),
            None => Err(DeviceError::UnsupportedAction),
        }
    }

    fn wrapper_process(&mut self) -> Result<DeviceProcessOutputWrapper, DeviceError> {
        self.process().map(DeviceProcessOutputWrapper::from)
    }
}

pub trait DeviceActionTrait: AsAny + Send {
    type Result: DeviceActionResultTrait; // TODO Check if Clone trait can be added here
}

pub trait DeviceActionResultTrait: AsAny + Send {}

pub trait DeviceActionWrapperTrait: AsAny + Send {}

impl<T> DeviceActionWrapperTrait for T where T: DeviceActionTrait {}
