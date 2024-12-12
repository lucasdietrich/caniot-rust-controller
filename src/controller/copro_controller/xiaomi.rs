use ble_copro_stream_server::xiaomi::XiaomiRecord;

use super::devices::BleMeasurement;

impl Into<BleMeasurement> for XiaomiRecord {
    fn into(self) -> BleMeasurement {
        BleMeasurement::new(
            self.timestamp.to_utc(),
            self.measurement.rssi,
            Some(self.measurement.battery_mv),
            Some(self.measurement.battery_percent),
            Some(self.measurement.temperature),
            Some(self.measurement.humidity),
        )
    }
}
