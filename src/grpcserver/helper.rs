use chrono::{DateTime, Local, NaiveTime, Utc};
use prost_types::Timestamp;
use std::time::SystemTime;

pub fn utc_to_prost_timestamp(dt: &DateTime<Utc>) -> Timestamp {
    let ts = dt.timestamp_nanos_opt().unwrap_or_default();

    Timestamp {
        seconds: ts / 1_000_000_000,
        nanos: (ts % 1_000_000_000) as i32,
    }
}

pub fn local_to_prost_timestamp(dt: &DateTime<Local>) -> Timestamp {
    let ts = dt.timestamp_nanos_opt().unwrap_or_default();

    Timestamp {
        seconds: ts / 1_000_000_000,
        nanos: (ts % 1_000_000_000) as i32,
    }
}

pub fn naive_time_to_string(dt: &NaiveTime) -> String {
    dt.format("%H:%M:%S").to_string()
}

pub fn string_to_naive_time(s: &str) -> NaiveTime {
    NaiveTime::parse_from_str(s, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
        .or_else(|_| NaiveTime::parse_from_str(s, "%H"))
        .expect("Failed to parse time")
}

pub fn systemtime_to_prost_timestamp(time: SystemTime) -> Timestamp {
    let duration = time
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    }
}
