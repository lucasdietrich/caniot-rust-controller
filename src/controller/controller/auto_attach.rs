// Handle device controller attachment and detachment.

use crate::{
    caniot::DeviceId,
    controller::{
        AlarmController, DemoController, Device, DeviceControllerWrapperTrait, GarageController,
        HeatersController,
    },
};

pub const DEVICE_DEMO_DID: u8 = 0;
pub const DEVICE_HEATERS_DID: u8 = 1;
pub const DEVICE_GARAGE_DID: u8 = 0x10;
pub const DEVICE_OUTDOOR_ALARM_DID: u8 = 0x18;

pub fn device_get_controller_impl(did: &DeviceId) -> Option<Box<dyn DeviceControllerWrapperTrait>> {
    match did.to_u8() {
        DEVICE_DEMO_DID => Some(Box::new(DemoController::default())),
        DEVICE_HEATERS_DID => Some(Box::new(HeatersController::default())),
        DEVICE_GARAGE_DID => Some(Box::new(GarageController::default())),
        DEVICE_OUTDOOR_ALARM_DID => Some(Box::new(AlarmController::default())),
        _ => None,
    }
}

pub fn device_attach_controller(
    device: &mut Device,
) -> Option<Box<dyn DeviceControllerWrapperTrait>> {
    if let Some(controller) = device_get_controller_impl(&device.did) {
        device.controller.replace(controller)
    } else {
        None
    }
}
