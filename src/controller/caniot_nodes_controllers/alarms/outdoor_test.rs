use chrono::Duration;

use super::*;

#[test]
fn auto_lights_range() {
    let context = NightLightsContext {
        auto_active: true,
        desired_duration: Duration::seconds(60),
    };

    assert_eq!(context.is_active(), true);
}
