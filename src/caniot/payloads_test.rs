#[test]
fn set_xps() {
    use super::Xps;
    fn test(xps: Xps, len: usize, pos: usize, expected: &[u8]) {
        let mut data = [0_u8; 8];
        let data = &mut data[..len];
        xps.set_at(data, pos).unwrap();
        assert_eq!(&data[..len], expected);
    }

    test(Xps::None, 1, 1, &[0b0000_0000]);
    test(Xps::PulseCancel, 1, 0, &[0b0000_0111]);
    test(Xps::PulseCancel, 1, 1, &[0b0011_1000]);
    test(Xps::PulseCancel, 1, 2, &[0b1100_0000]);
    test(Xps::PulseCancel, 2, 2, &[0b1100_0000, 0b0000_0001]);
    test(Xps::PulseCancel, 2, 3, &[0b0000_0000, 0b0000_1110]);
    test(Xps::PulseCancel, 2, 4, &[0b0000_0000, 0b0111_0000]);
    test(Xps::PulseCancel, 2, 5, &[0b0000_0000, 0b1000_0000]);
    test(
        Xps::PulseCancel,
        3,
        5,
        &[0b0000_0000, 0b1000_0000, 0b0000_0011],
    );
    test(
        Xps::PulseCancel,
        3,
        6,
        &[0b0000_0000, 0b0000_0000, 0b0001_1100],
    );
    test(
        Xps::PulseCancel,
        3,
        7,
        &[0b0000_0000, 0b0000_0000, 0b1110_0000],
    );
}
