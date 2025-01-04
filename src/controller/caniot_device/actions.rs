use crate::caniot::{self, Response};

use super::{ActionResultTrait, ActionTrait, ActionWrapperTrait};

#[derive(Debug)]
pub enum DeviceAction {
    // Reset device
    Reset,
    // Reset device settings to factory defaults
    ResetSettings,
    // Inhibit device from performing control actions (e.g. siren, lights
    InhibitControl(caniot::TSP),
    // Ping (request telemetry)
    Ping(caniot::Endpoint),
    // Action to pass to the underlying device
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
    // Action has been completed
    Done,
    // Reset command has been sent to the device and telemetry has been received
    ResetSent,
    // Reset settings command has been sent to the device and telemetry has been received
    ResetSettingsSent,
    // Inhibit control command has been sent to the device and telemetry has been received
    InhibitControlSent,
    // Pong response from the device
    Pong(Response),
    // Inner action result
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
