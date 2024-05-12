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

    let temps = [
        Temperature::new(-1100),
        Temperature::new(0),
        Temperature::new(3300),
        Temperature::new(4400),
    ];

    for i in 0..=2 {
        for temp in temps.iter() {
            let mut telem = class0::Telemetry::default();
            telem.temp_out[i] = *temp;
            let ser: Vec<u8> = telem.into();

            let deser = class0::Telemetry::try_from(ser.as_slice());
            assert_eq!(deser.is_ok(), true);

            let deser = deser.unwrap();
            assert_eq!(deser.temp_out[i], *temp);
        }
    }
}
