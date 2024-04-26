use std::fmt::Debug;

use crate::{
    caniot::{self},
    controller::DeviceActionTrait,
};

use as_any::Downcast;
use thiserror::Error;

use super::DeviceResult;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Invalid action")]
    InvalidAction,
    #[error("NoInnerDevice")]
    NoInnerDevice,
    #[error("Invalid frame")]
    InvalidFrame,
}

pub trait DeviceTrait: Send + Debug {
    type Action;

    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<DeviceResult, DeviceError>;
    fn handle_action(&mut self, action: &Self::Action) -> Result<DeviceResult, DeviceError>;
    fn process(&mut self) -> Result<DeviceResult, DeviceError>;
}

/// This trait is used to wrap a DeviceTrait into a DeviceWrapperTrait and make it object safe
pub trait DeviceWrapperTrait: Send + Debug {
    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<DeviceResult, DeviceError>;
    fn handle_action(
        &mut self,
        action: &Box<dyn DeviceActionTrait>,
    ) -> Result<DeviceResult, DeviceError>;
    fn process(&mut self) -> Result<DeviceResult, DeviceError>;
}

impl<T: DeviceTrait> DeviceWrapperTrait for T
where
    <T as DeviceTrait>::Action: 'static,
{
    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<DeviceResult, DeviceError> {
        self.handle_frame(frame)
    }

    fn handle_action(
        &mut self,
        action: &Box<dyn DeviceActionTrait>,
    ) -> Result<DeviceResult, DeviceError> {
        match action.downcast_ref::<T::Action>() {
            Some(action) => self.handle_action(action),
            None => Err(DeviceError::InvalidAction),
        }
    }

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        self.process()
    }
}
