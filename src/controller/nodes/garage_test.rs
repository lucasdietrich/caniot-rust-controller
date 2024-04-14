use super::*;
use crate::caniot::*;

#[test]
fn enc() {
    let cmd = GarageDoorCommand {
        left_door_activate: true,
        right_door_activate: true,
    };
    let cmd: Class0Command = cmd.into();
    assert_eq!(cmd.crl1, Xps::PulseOn);
    assert_eq!(cmd.crl2, Xps::PulseOn);
    assert_eq!(cmd.coc1, Xps::None);
    assert_eq!(cmd.coc2, Xps::None);
}

#[test]
fn dec() {
    let payload = Class0Payload {
        in2: true,
        in3: true,
        in4: true,
        ..Default::default()
    };
    let status = GarageDoorStatus::from(payload);
    assert_eq!(status.left_door_status, true);
    assert_eq!(status.right_door_status, true);
    assert_eq!(status.garage_light_status, true);
}
