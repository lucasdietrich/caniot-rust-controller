use std::time::Duration;

use crate::{
    bus::CanInterface,
    caniot::{self, DeviceId},
};

use super::{heaters::HeatersController, nodes, Class0Behavior, Class1Behavior, Device};

pub fn emu_pool1_add_devices_to_iface(iface: &mut CanInterface) {
    let mut dev1 = Device::new(1, Some(Duration::from_secs(5)));
    dev1.add_behavior(Box::new(super::CounterBehavior::default()));

    let mut dev2 = Device::new(2, Some(Duration::from_secs(5)));
    dev2.add_behavior(Box::new(super::EchoBehavior::default()));

    let mut dev3 = Device::new(3, Some(Duration::from_secs(5)));
    dev3.add_behavior(Box::new(super::RandomBehavior::default()));

    iface.add_device(dev1);
    iface.add_device(dev2);
    iface.add_device(dev3);

    (10..=20)
        .map(|i| {
            let mut dev = Device::new(i, Some(Duration::from_secs(5)));
            dev.add_behavior(Box::new(super::RandomBehavior::default()));
            dev
        })
        .for_each(|dev| iface.add_device(dev));
}

pub fn emu_pool2_realistic_add_devices_to_iface(iface: &mut CanInterface) {
    // Add heaters controllers at
    let mut heaters_controller = Device::new(
        DeviceId::new(1, 0).unwrap().to_u8(),
        Some(Duration::from_secs(30)),
    );
    heaters_controller.request_telemetry(caniot::Endpoint::ApplicationDefault);
    heaters_controller.set_telemetry_endpoint(caniot::Endpoint::BoardControl);
    heaters_controller.add_behavior(Box::new(Class1Behavior::default()));
    heaters_controller.add_behavior(Box::new(HeatersController::new()));
    iface.add_device(heaters_controller);

    // Add demo device
    let mut demo_controller = Device::new(
        DeviceId::new(0, 0).unwrap().to_u8(),
        Some(Duration::from_secs(30)),
    );
    demo_controller.add_behavior(Box::new(Class0Behavior::default()));
    demo_controller.set_telemetry_endpoint(caniot::Endpoint::BoardControl);
    demo_controller.add_behavior(Box::new(super::demo::DemoController::new()));
    iface.add_device(demo_controller);

    // Add garge device
    let mut garage_controller = Device::new(
        DeviceId::new(0, 2).unwrap().to_u8(),
        Some(Duration::from_secs(30)),
    );
    garage_controller.add_behavior(Box::new(nodes::garage::GarageController::default()));
    garage_controller.set_telemetry_endpoint(caniot::Endpoint::BoardControl);
    iface.add_device(garage_controller);
}
