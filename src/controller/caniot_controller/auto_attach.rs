// Handle device controller attachment and detachment.

use crate::{
    controller::{
        AlarmConfig, AlarmController, CaniotDevicesConfig, ConfigTrait, DemoController,
        DeviceControllerTrait, DeviceControllerWrapperTrait, GarageController, HeatersController,
    },
    database::SettingsStore,
};
use caniot::DeviceId;

pub const DEVICE_DEMO_DID: u8 = 0;
pub const DEVICE_HEATERS_DID: u8 = 1;
pub const DEVICE_GARAGE_DID: u8 = 0x10;
pub const DEVICE_OUTDOOR_ALARM_DID: u8 = 0x18;

pub async fn device_init_controller<'stg>(
    did: DeviceId,
    _device_infos: (),
    devices_config: &CaniotDevicesConfig,
    stg: SettingsStore<'stg>,
) -> Option<Box<dyn DeviceControllerWrapperTrait>> {
    if did == DeviceId::try_from(devices_config.demo_did.unwrap_or(DEVICE_DEMO_DID)).unwrap() {
        Some(Box::new(DemoController::new(None)))
    } else if did
        == DeviceId::try_from(devices_config.heaters_did.unwrap_or(DEVICE_HEATERS_DID)).unwrap()
    {
        Some(Box::new(HeatersController::new(None)))
    } else if did
        == DeviceId::try_from(devices_config.garage_did.unwrap_or(DEVICE_GARAGE_DID)).unwrap()
    {
        Some(Box::new(GarageController::new(None)))
    } else if did
        == DeviceId::try_from(
            devices_config
                .outdoor_alarm_did
                .unwrap_or(DEVICE_OUTDOOR_ALARM_DID),
        )
        .unwrap()
    {
        let alarm_config = AlarmConfig::load(&stg).await.unwrap_or_default();
        Some(Box::new(AlarmController::new(Some(&alarm_config))))
    } else {
        None
    }
}
