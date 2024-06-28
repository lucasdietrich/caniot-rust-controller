use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::caniot::{self, BoardClassTelemetry, Response};

use as_any::{AsAny, Downcast};

use super::{
    alert::DeviceAlert,
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
            self.get_infos()
        );
        Err(DeviceError::NotImplemented)
    }

    // Building an action result shouldn't alter the device state (i.e. &self only)
    fn handle_action_result(
        &self,
        _delayed_action: &Self::Action,
        _completed_by: Response,
    ) -> Result<<Self::Action as ActionTrait>::Result, DeviceError> {
        error!(
            "handle_action_result not implemented for device controller \"{}\"",
            self.get_infos()
        );
        Err(DeviceError::NotImplemented)
    }

    fn process(&mut self, _ctx: &mut ProcessContext) -> Result<Verdict, DeviceError> {
        Ok(Verdict::default())
    }

    // Retrieve device controller infos
    fn get_infos(&self) -> DeviceControllerInfos;

    // Retrieve active alert if any
    fn get_alert(&self) -> Option<DeviceAlert> {
        None
    }
}

#[derive(Debug, Default)]
pub struct DeviceControllerInfos {
    pub name: String,
    pub display_name: Option<String>,

    // Name of the controller view in the UI
    pub ui_view_name: Option<String>,
}

impl DeviceControllerInfos {
    pub fn new(name: &str, display_name: Option<&str>, ui_view_name: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            ui_view_name: ui_view_name.map(|s| s.to_string()),
        }
    }
}

impl Display for DeviceControllerInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(display_name) = &self.display_name {
            write!(f, "{} ({})", display_name, self.name)
        } else {
            write!(f, "{}", self.name)
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
        _completed_by: caniot::Response,
    ) -> Result<Box<dyn ActionResultTrait>, DeviceError> {
        Err(DeviceError::NotImplemented)
    }

    fn wrapper_process(&mut self, ctx: &mut ProcessContext) -> Result<Verdict, DeviceError>;

    fn wrapper_get_infos(&self) -> DeviceControllerInfos;

    fn wrapper_get_alert(&self) -> Option<DeviceAlert>;
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
        completed_by: Response,
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

    fn wrapper_get_alert(&self) -> Option<DeviceAlert> {
        self.get_alert()
    }
}

pub trait ActionTrait: AsAny + Send + Debug {
    type Result: ActionResultTrait; // TODO Check if Clone trait can be added here
}

pub trait ActionResultTrait: AsAny + Send {}

pub trait ActionWrapperTrait: AsAny + Send + Debug {}

impl<T> ActionWrapperTrait for T where T: ActionTrait + Debug {}
