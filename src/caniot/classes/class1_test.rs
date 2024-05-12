use crate::caniot::Temperature;

use super::class1;

#[test]
fn telemetry() {
    let telem = class1::Telemetry::default();
    let ser: Vec<u8> = telem.into();
    assert_eq!(ser.len(), 8);
    assert_eq!(ser, vec![0, 0, 0, 255, 255, 255, 255, 255]);

    for i in 0..class1::Telemetry::default().ios.len() {
        let mut telem = class1::Telemetry::default();
        telem.ios[i] = true;
        let ser: Vec<u8> = telem.into();
        assert_eq!(ser.len(), 8);
        assert_eq!(ser[i / 8], 1 << (i % 8));
    }
}
