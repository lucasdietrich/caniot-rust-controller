// Handle device controller attachment and detachment.

use crate::controller::{
    AlarmController, CaniotDevicesConfig, DemoController, Device, DeviceControllerTrait,
    DeviceControllerWrapperTrait, GarageController, HeatersController,
};

pub const DEVICE_DEMO_DID: u8 = 0;
pub const DEVICE_HEATERS_DID: u8 = 1;
pub const DEVICE_GARAGE_DID: u8 = 0x10;
pub const DEVICE_OUTDOOR_ALARM_DID: u8 = 0x18;

pub fn device_attach_controller(
    device: &mut Device,
    config: &CaniotDevicesConfig,
) -> Option<Box<dyn DeviceControllerWrapperTrait>> {
    let did = device.did.to_u8();
    let implementation: Option<Box<dyn DeviceControllerWrapperTrait>> =
        if did == config.demo_did.unwrap_or(DEVICE_DEMO_DID) {
            Some(Box::new(DemoController::new(None)))
        } else if did == config.heaters_did.unwrap_or(DEVICE_HEATERS_DID) {
            Some(Box::new(HeatersController::new(None)))
        } else if did == config.garage_did.unwrap_or(DEVICE_GARAGE_DID) {
            Some(Box::new(GarageController::new(None)))
        } else if did == config.outdoor_alarm_did.unwrap_or(DEVICE_OUTDOOR_ALARM_DID) {
            Some(Box::new(AlarmController::new(Some(&config.alarm_config))))
        } else {
            None
        };

    if let Some(controller) = implementation {
        device.controller.replace(controller)
    } else {
        None
    }
}
