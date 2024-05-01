use super::{DeviceActionResultTrait, DeviceActionTrait, DeviceActionWrapperTrait};

pub enum DeviceAction {
    Reset,
    Inner(Box<dyn DeviceActionWrapperTrait>),
}

impl DeviceAction {
    pub fn new_inner<A: DeviceActionTrait>(action: A) -> Self {
        Self::Inner(Box::new(action))
    }
}

impl DeviceActionTrait for DeviceAction {
    type Result = DeviceActionResult;
}

// #[derive(Clone)]
pub enum DeviceActionResult {
    Reset,
    Inner(Box<dyn DeviceActionResultTrait>),
}

impl DeviceActionResult {
    pub fn new_inner<R: DeviceActionResultTrait>(result: R) -> Self {
        Self::Inner(Box::new(result))
    }

    pub fn new_boxed_inner(result: Box<dyn DeviceActionResultTrait>) -> Self {
        Self::Inner(result)
    }
}

impl DeviceActionResultTrait for DeviceActionResult {}
