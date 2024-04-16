use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use std::time::SystemTime;

pub fn datetime_to_prost_timestamp(dt: &DateTime<Utc>) -> prost_types::Timestamp {
    let ts = dt.timestamp_nanos_opt().unwrap_or_default();

    Timestamp {
        seconds: ts / 1_000_000_000,
        nanos: (ts % 1_000_000_000) as i32,
    }
}

pub fn systemtime_to_prost_timestamp(time: SystemTime) -> Timestamp {
    let duration = time.duration_since(std::time::UNIX_EPOCH).unwrap();

    Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    }
}
