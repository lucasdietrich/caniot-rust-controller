use std::time::Duration;

use crate::{
    bus::CanInterface,
    caniot::{self, DeviceId},
};

use super::{Device, HeatersController};

pub fn emu_pool1_add_devices_to_iface(iface: &mut CanInterface) {
    let mut dev1 = Device::new(1, Duration::from_secs(5));
    dev1.add_behavior(Box::new(super::CounterBehavior::default()));

    let mut dev2 = Device::new(2, Duration::from_secs(5));
    dev2.add_behavior(Box::new(super::EchoBehavior::default()));

    let mut dev3 = Device::new(3, Duration::from_secs(5));
    dev3.add_behavior(Box::new(super::RandomBehavior::default()));

    iface.add_device(dev1);
    iface.add_device(dev2);
    iface.add_device(dev3);

    (10..=20)
        .map(|i| {
            let mut dev = Device::new(i, Duration::from_secs(5));
            dev.add_behavior(Box::new(super::RandomBehavior::default()));
            dev
        })
        .for_each(|dev| iface.add_device(dev));
}

pub fn emu_pool2_realistic_add_devices_to_iface(iface: &mut CanInterface) {
    // Add heaters controllers at
    let mut heaters_controller = Device::new(
        DeviceId::new(1, 0).unwrap().to_u8(),
        Duration::from_secs(30),
    );
    let heaters_behavior = HeatersController::new();
    heaters_controller.set_telemetry_endpoint(caniot::Endpoint::ApplicationDefault);
    heaters_controller.add_behavior(Box::new(heaters_behavior));
    iface.add_device(heaters_controller);
}
