use std::{fmt::Debug, ops::Deref};

use crate::caniot;

use as_any::{AsAny, Downcast};

use super::{DeviceError, DeviceEvent, DeviceProcessOutput, DeviceProcessOutputWrapper};

pub trait DeviceTrait: Send + Debug {
    type Action: DeviceActionTrait;

    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError>;

    fn handle_action(
        &mut self,
        action: &Self::Action,
    ) -> Result<DeviceProcessOutput<Self::Action>, DeviceError>;

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
pub trait DeviceWrapperTrait: Send + Debug {
    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutputWrapper, DeviceError>;
    fn handle_action(
        &mut self,
        action: &Box<dyn DeviceActionWrapperTrait>,
    ) -> Result<DeviceProcessOutputWrapper, DeviceError>;
    fn process(&mut self) -> Result<DeviceProcessOutputWrapper, DeviceError>;
}

impl<T: DeviceTrait> DeviceWrapperTrait for T
where
    <T as DeviceTrait>::Action: 'static,
{
    fn handle_frame(
        &mut self,
        frame: &caniot::ResponseData,
    ) -> Result<DeviceProcessOutputWrapper, DeviceError> {
        self.handle_frame(frame)
            .map(DeviceProcessOutputWrapper::from)
    }

    fn handle_action(
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

    fn process(&mut self) -> Result<DeviceProcessOutputWrapper, DeviceError> {
        self.process().map(DeviceProcessOutputWrapper::from)
    }
}

pub trait DeviceActionTrait: AsAny + Send {
    type Result: DeviceActionResultTrait;
}

pub trait DeviceActionResultTrait: AsAny + Send {}

pub trait DeviceActionWrapperTrait: AsAny + Send {}

impl<T> DeviceActionWrapperTrait for T where T: DeviceActionTrait {}
