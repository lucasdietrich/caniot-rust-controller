// Handle device controller attachment and detachment.

use crate::{
    caniot::DeviceId,
    controller::{DemoController, Device, DeviceWrapperTrait, HeatersController},
};

pub const DEVICE_DEMO_DID: u8 = 0;
pub const DEVICE_HEATERS_DID: u8 = 1;

pub fn device_get_controller_impl(did: &DeviceId) -> Option<Box<dyn DeviceWrapperTrait>> {
    match did.to_u8() {
        DEVICE_DEMO_DID => Some(Box::new(DemoController::default())),
        DEVICE_HEATERS_DID => Some(Box::new(HeatersController::default())),
        _ => None,
    }
}

pub fn device_attach_controller(device: &mut Device) -> Option<Box<dyn DeviceWrapperTrait>> {
    if let Some(controller) = device_get_controller_impl(&device.did) {
        device.inner.replace(controller)
    } else {
        None
    }
}
