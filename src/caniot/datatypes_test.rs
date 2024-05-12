use super::{Temperature, Xps};

#[test]
fn set_xps() {
    fn set_cmp(xps: Xps, len: usize, pos: usize, expected: &[u8]) {
        let mut data = [0_u8; 8];
        let data = &mut data[..len];
        assert!(xps.set_at(data, pos).is_ok());
        assert_eq!(&data[..len], expected);
    }

    set_cmp(Xps::None, 1, 1, &[0b0000_0000]);
    set_cmp(Xps::PulseCancel, 1, 0, &[0b0000_0111]);
    set_cmp(Xps::PulseCancel, 1, 1, &[0b0011_1000]);
    set_cmp(Xps::PulseCancel, 1, 2, &[0b1100_0000]);
    set_cmp(Xps::PulseCancel, 2, 2, &[0b1100_0000, 0b0000_0001]);
    set_cmp(Xps::PulseCancel, 2, 3, &[0b0000_0000, 0b0000_1110]);
    set_cmp(Xps::PulseCancel, 2, 4, &[0b0000_0000, 0b0111_0000]);
    set_cmp(Xps::PulseCancel, 2, 5, &[0b0000_0000, 0b1000_0000]);
    set_cmp(
        Xps::PulseCancel,
        3,
        5,
        &[0b0000_0000, 0b1000_0000, 0b0000_0011],
    );
    set_cmp(
        Xps::PulseCancel,
        3,
        6,
        &[0b0000_0000, 0b0000_0000, 0b0001_1100],
    );
    set_cmp(
        Xps::PulseCancel,
        3,
        7,
        &[0b0000_0000, 0b0000_0000, 0b1110_0000],
    );
}

#[test]
fn get_xps() {
    fn get_cmp(payload: &[u8], pos: usize, expected: Xps) {
        let result = Xps::get_at(payload, pos);
        assert!(result.is_ok());

        let xps = result.unwrap();
        assert_eq!(xps, expected);
    }

    get_cmp(&[0b0000_0000], 1, Xps::None);
    get_cmp(&[0b0000_0111], 0, Xps::PulseCancel);
    get_cmp(&[0b0011_1000], 1, Xps::PulseCancel);
    get_cmp(&[0b1100_0000, 0b0000_0001], 2, Xps::PulseCancel);
    get_cmp(&[0b0000_0000, 0b0000_1110], 3, Xps::PulseCancel);
    get_cmp(&[0b0000_0000, 0b0111_0000], 4, Xps::PulseCancel);
    get_cmp(
        &[0b0000_0000, 0b1000_0000, 0b0000_0011],
        5,
        Xps::PulseCancel,
    );
    get_cmp(
        &[0b0000_0000, 0b0000_0000, 0b0001_1100],
        6,
        Xps::PulseCancel,
    );
    get_cmp(
        &[0b0000_0000, 0b0000_0000, 0b1110_0000],
        7,
        Xps::PulseCancel,
    );
}

#[test]
fn temperature() {
    assert_eq!(Temperature::from_raw_u10(0).to_celsius(), Some(-28.0));
    assert_eq!(Temperature::from_raw_u10(1000).to_celsius(), Some(72.0));
    assert_eq!(Temperature::from_raw_u10(1001), Temperature::INVALID);
    assert_eq!(Temperature::from_raw_u10(0).to_raw_u10(), 0);
    assert_eq!(Temperature::from_raw_u10(1000).to_raw_u10(), 1000);
    assert_eq!(
        Temperature::from_raw_u10(1001).to_raw_u10(),
        Temperature::INVALID.to_raw_u10()
    );
    assert_eq!(
        Temperature::from_raw_u10(1023).to_raw_u10(),
        Temperature::INVALID.to_raw_u10()
    );
}
