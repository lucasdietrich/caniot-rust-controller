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
    #[error("Unsupported action for device")]
    UnsupportedAction,
    #[error("NoInnerDevice")]
    NoInnerDevice,
    #[error("Invalid frame")]
    InvalidFrame,
}

pub enum DeviceEvent<A> {
    Process,
    Action(A),
    Frame(caniot::ResponseData),
}

pub trait DeviceTrait: Send + Debug {
    type Action;

    // fn set_did(&mut self, did: caniot::DeviceId);
    // fn get_did(&self) -> caniot::DeviceId;

    fn handle_frame(&mut self, frame: &caniot::ResponseData) -> Result<DeviceResult, DeviceError>;
    fn handle_action(&mut self, action: &Self::Action) -> Result<DeviceResult, DeviceError>;

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        Ok(DeviceResult::default())
    }

    fn handle_event(
        &mut self,
        event: &DeviceEvent<Self::Action>,
    ) -> Result<DeviceResult, DeviceError> {
        match event {
            DeviceEvent::Process => self.process(),
            DeviceEvent::Action(action) => self.handle_action(action),
            DeviceEvent::Frame(frame) => self.handle_frame(frame),
        }
    }
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
            None => Err(DeviceError::UnsupportedAction),
        }
    }

    fn process(&mut self) -> Result<DeviceResult, DeviceError> {
        self.process()
    }
}
