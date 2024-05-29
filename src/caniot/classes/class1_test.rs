use num::FromPrimitive;

use crate::caniot::{Temperature, Xps};

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

    let temps = [
        Temperature::new(-1100),
        Temperature::new(0),
        Temperature::new(3300),
        Temperature::new(4400),
    ];

    for i in 0..=2 {
        for temp in temps.iter() {
            let mut telem = class1::Telemetry::default();
            telem.temp_out[i] = *temp;
            let ser: Vec<u8> = telem.into();

            let deser = class1::Telemetry::try_from(ser.as_slice());
            assert_eq!(deser.is_ok(), true);

            let deser = deser.unwrap();
            assert_eq!(deser.temp_out[i], *temp);
        }
    }
}

#[test]
fn command() {
    let mut cmd = class1::Command::default();
    for i in 0..class1::CLASS1_IO_COUNT {
        FromPrimitive::from_u8((i & 0x7) as u8).map(|x| cmd.ios[i] = x);
    }

    let ser: Vec<u8> = cmd.into();
    assert_eq!(ser.len(), 7);

    let deser = class1::Command::try_from(ser.as_slice());
    assert_eq!(deser.is_ok(), true);

    let deser = deser.unwrap();
    for i in 0..class1::CLASS1_IO_COUNT {
        assert_eq!(deser.ios[i], cmd.ios[i]);
    }
}
