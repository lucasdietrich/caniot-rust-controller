use std::time::Duration;

use super::helpers::*;

#[test]
fn test_xps_simple() {
    let mut xps = EmuXps::new(false, false, None);
    assert_eq!(xps.get_state(), false);
    assert_eq!(xps.supports_pulse(), false);
    assert_eq!(xps.pulse_pending(), false);
    assert_eq!(xps.pulse_expired(), false);
    assert_eq!(xps.time_to_pulse_expire(), None);
    assert_eq!(xps.pulse_process(), None);

    xps.apply(&crate::caniot::Xps::SetOn);
    assert_eq!(xps.get_state(), true);

    xps.apply(&crate::caniot::Xps::SetOff);
    assert_eq!(xps.get_state(), false);

    xps.apply(&crate::caniot::Xps::SetOn);
    assert_eq!(xps.get_state(), true);

    xps.apply(&crate::caniot::Xps::None);
    assert_eq!(xps.get_state(), true);

    xps.apply(&crate::caniot::Xps::Reset);
    assert_eq!(xps.get_state(), false);
}

#[test]
fn test_xps_pulse() {
    let duration = Duration::from_millis(100);
    let mut xps = EmuXps::new(false, false, Some(duration));

    assert_eq!(xps.get_state(), false);
    assert_eq!(xps.supports_pulse(), true);
    assert_eq!(xps.pulse_pending(), false);
    assert_eq!(xps.pulse_expired(), false);
    assert_eq!(xps.time_to_pulse_expire(), None);
    assert_eq!(xps.pulse_process(), None);

    xps.apply(&crate::caniot::Xps::PulseOn);
    assert_eq!(xps.get_state(), true);
    assert_eq!(xps.pulse_pending(), true);
    assert_eq!(xps.pulse_expired(), false);

    // This sleep is bad (todo, create an "advance()" method to manually advance in time)
    std::thread::sleep(duration);

    assert_eq!(xps.pulse_expired(), true);
    assert_eq!(xps.pulse_process(), Some(false));
    assert_eq!(xps.get_state(), false);
    assert_eq!(xps.pulse_pending(), false);
    assert_eq!(xps.pulse_expired(), false);
    assert_eq!(xps.time_to_pulse_expire(), None);
}
