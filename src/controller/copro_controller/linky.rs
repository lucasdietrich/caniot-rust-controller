use ble_copro_stream_server::linky::LinkyTicRecord;
use chrono::{DateTime, Utc};

use crate::controller::copro_controller::measurements::{
    EnergyMeterTrait, RssiTrait, TimestampedTrait,
};

impl TimestampedTrait for LinkyTicRecord {
    fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp.to_utc()
    }
}

impl RssiTrait for LinkyTicRecord {
    fn rssi(&self) -> i8 {
        self.rssi
    }
}

impl EnergyMeterTrait for LinkyTicRecord {
    fn power(&self) -> Option<f32> {
        Some(self.measurement.papp as f32)
    }

    fn current(&self) -> Option<f32> {
        Some(self.measurement.iinst as f32)
    }

    fn energy(&self) -> Option<f32> {
        Some(self.measurement.base as f32)
    }
}
