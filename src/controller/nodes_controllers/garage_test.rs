use super::*;
use crate::caniot::*;

#[test]
fn enc() {
    let cmd = GarageDoorCommand {
        left_door_activate: true,
        right_door_activate: true,
    };
    let cmd: class0::Command = (&cmd).into();
    assert_eq!(cmd.crl1, Xps::PulseOn);
    assert_eq!(cmd.crl2, Xps::PulseOn);
    assert_eq!(cmd.coc1, Xps::None);
    assert_eq!(cmd.coc2, Xps::None);
}

#[test]
fn dec() {
    let payload = class0::Telemetry {
        in2: false,
        in3: false,
        in4: false,
        ..Default::default()
    };
    let ios = GarageIOState::from(&payload);
    let status = GarageDoorStatus::init(ios);
    assert_eq!(status.left_door_status.get(), DoorState::Closed);
    assert_eq!(status.right_door_status.get(), DoorState::Closed);
    assert_eq!(status.gate_open.get(), false);
}
