use chrono::{Duration, NaiveTime};

use super::*;

#[test]
fn auto_lights_range() {
    let context = NightLightsContext {
        auto: true,
        auto_range: [
            NaiveTime::from_hms_opt(20, 0, 0).expect("Invalid time"),
            NaiveTime::from_hms_opt(6, 0, 0).expect("Invalid time"),
        ],
        desired_duration: Duration::seconds(60),
    };

    let test_data = [
        (NaiveTime::from_hms_opt(19, 59, 59).unwrap(), false),
        (NaiveTime::from_hms_opt(20, 0, 0).unwrap(), true),
        (NaiveTime::from_hms_opt(23, 0, 0).unwrap(), true),
        (NaiveTime::from_hms_opt(0, 0, 0).unwrap(), true),
        (NaiveTime::from_hms_opt(5, 59, 59).unwrap(), true),
        (NaiveTime::from_hms_opt(6, 0, 0).unwrap(), false),
        (NaiveTime::from_hms_opt(12, 0, 0).unwrap(), false),
    ];

    for (time, expected) in test_data.iter() {
        assert_eq!(context.is_active(time), *expected);
    }
}
