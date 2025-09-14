use caniot::{class::llpayload::LLCommand, Xps};

use super::*;

#[test]
fn enc() {
    let cmd = GarageDoorCommand {
        left_door_activate: true,
        right_door_activate: true,
    };
    let cmd: caniot::class0::Command = (&cmd).into();
    assert_eq!(
        cmd.get_io_xps(caniot::class0::IO::Relay1).unwrap(),
        Xps::PulseOn,
    );
    assert_eq!(
        cmd.get_io_xps(caniot::class0::IO::Relay2).unwrap(),
        Xps::PulseOn,
    );
    assert_eq!(cmd.get_io_xps(caniot::class0::IO::Oc1).unwrap(), Xps::None);
    assert_eq!(cmd.get_io_xps(caniot::class0::IO::Oc2).unwrap(), Xps::None);
}

#[test]
fn dec() {
    let mut payload = caniot::class0::Telemetry::default();
    let data = payload.to_telemetry_data().unwrap();
    let ios = GarageIOState::from(&data);
    let status = GarageDoorStatus::init(ios);
    assert_eq!(status.left_door_status.get(), DoorState::Closed);
    assert_eq!(status.right_door_status.get(), DoorState::Closed);
    assert_eq!(status.gate_open.get(), false);
}
