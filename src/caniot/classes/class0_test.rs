use crate::caniot::Temperature;

use super::class0;

#[test]
fn telemetry() {
    let telem = class0::Telemetry::default();
    let ser: Vec<u8> = telem.into();
    assert_eq!(ser.len(), 7);
    assert_eq!(ser, vec![0, 0, 255, 255, 255, 255, 255]);

    let mut telem = class0::Telemetry::default();
    telem.oc1 = true;
    let ser: Vec<u8> = telem.into();
    assert_eq!(ser, vec![1, 0, 255, 255, 255, 255, 255]);

    let mut telem = class0::Telemetry::default();
    telem.in1 = true;
    let ser: Vec<u8> = telem.into();
    assert_eq!(ser, vec![16, 0, 255, 255, 255, 255, 255]);
}
