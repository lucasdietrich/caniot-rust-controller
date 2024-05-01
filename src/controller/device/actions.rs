use super::{ActionResultTrait, ActionTrait, ActionWrapperTrait};

pub enum DeviceAction {
    Reset,
    Inner(Box<dyn ActionWrapperTrait>),
}

impl DeviceAction {
    pub fn new_inner<A: ActionTrait>(action: A) -> Self {
        Self::Inner(Box::new(action))
    }
}

impl ActionTrait for DeviceAction {
    type Result = DeviceActionResult;
}

// #[derive(Clone)]
pub enum DeviceActionResult {
    Reset,
    Inner(Box<dyn ActionResultTrait>),
}

impl DeviceActionResult {
    pub fn new_inner<R: ActionResultTrait>(result: R) -> Self {
        Self::Inner(Box::new(result))
    }

    pub fn new_boxed_inner(result: Box<dyn ActionResultTrait>) -> Self {
        Self::Inner(result)
    }
}

impl ActionResultTrait for DeviceActionResult {}
