use ble_copro_stream_server::xiaomi::XiaomiRecord;
use chrono::{DateTime, Utc};

use crate::controller::copro_controller::measurements::{
    BatteryTrait, EnvironmentalTrait, RssiTrait, TimestampedTrait,
};

impl TimestampedTrait for XiaomiRecord {
    fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp.to_utc()
    }
}

impl EnvironmentalTrait for XiaomiRecord {
    fn temperature(&self) -> Option<f32> {
        Some(self.measurement.temperature)
    }

    fn humidity(&self) -> Option<f32> {
        Some(self.measurement.humidity)
    }
}

impl BatteryTrait for XiaomiRecord {
    fn battery_mv(&self) -> Option<u16> {
        Some(self.measurement.battery_mv)
    }

    fn battery_level(&self) -> Option<u8> {
        Some(self.measurement.battery_percent)
    }
}

impl RssiTrait for XiaomiRecord {
    fn rssi(&self) -> i8 {
        self.measurement.rssi
    }
}
