use chrono::{DateTime, Utc};
use core::fmt;

use super::core::AttrKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct U8Value {
    pub value: u8,
}

impl fmt::Display for U8Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / 0x{:02X}", self.value, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct U16Value {
    pub value: u16,
}

impl fmt::Display for U16Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / 0x{:04X}", self.value, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I16Value {
    pub value: i16,
}

impl fmt::Display for I16Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct U32Value {
    pub value: u32,
}

impl fmt::Display for U32Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / 0x{:08X}", self.value, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I32Value {
    pub value: i32,
}

impl fmt::Display for I32Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIdValue {
    pub raw: u8,
    pub class: u8,
    pub sid: u8,
}

impl fmt::Display for DeviceIdValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{:02X} (class {}, sid {})",
            self.raw, self.class, self.sid
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionValue {
    pub caniot: u8,
    pub application: u8,
}

impl fmt::Display for VersionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CANIOT {}, firmware {}", self.caniot, self.application)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextPartValue {
    pub part: u8,
    pub text: String,
}

impl fmt::Display for TextPartValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "part {}: {:?}", self.part, self.text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimestampValue {
    pub epoch_seconds: u32,
}

impl fmt::Display for TimestampValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.epoch_seconds == 0 {
            write!(f, "not set")
        } else if let Some(dt) = DateTime::<Utc>::from_timestamp(self.epoch_seconds as i64, 0) {
            write!(f, "{} UTC", dt.format("%Y-%m-%d %H:%M:%S"))
        } else {
            write!(f, "{} epoch-s", self.epoch_seconds)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondsValue {
    pub seconds: u32,
}

impl fmt::Display for SecondsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let days = self.seconds / 86_400;
        let hours = (self.seconds % 86_400) / 3_600;
        let minutes = (self.seconds % 3_600) / 60;
        let seconds = self.seconds % 60;

        if days > 0 {
            write!(f, "{}d {}h {}m {}s", days, hours, minutes, seconds)
        } else {
            write!(f, "{}h {}m {}s", hours, minutes, seconds)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MillisecondsValue {
    pub milliseconds: u32,
}

impl fmt::Display for MillisecondsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.milliseconds)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFlagsValue {
    pub error_response: bool,
    pub telemetry_delay_random: bool,
    pub telemetry_endpoint: u8,
    pub telemetry_periodic_enabled: bool,
}

impl fmt::Display for ConfigFlagsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error_response={}, telemetry_delay_random={}, telemetry_endpoint={}, telemetry_periodic_enabled={}",
            self.error_response as u8,
            self.telemetry_delay_random as u8,
            self.telemetry_endpoint,
            self.telemetry_periodic_enabled as u8
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimezoneValue {
    pub offset_seconds: i32,
}

impl fmt::Display for TimezoneValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.offset_seconds >= 0 { '+' } else { '-' };
        let abs = self.offset_seconds.unsigned_abs();
        let h = abs / 3_600;
        let m = (abs % 3_600) / 60;

        write!(f, "{}{:02}:{:02}", sign, h, m)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationValue {
    pub region: String,
    pub country: String,
}

impl fmt::Display for LocationValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.region, self.country)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResetReasonValue {
    pub reason: &'static str,
}

impl fmt::Display for ResetReasonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.reason)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesPartValue {
    pub part: u8,
    pub bytes: Vec<u8>,
}

impl fmt::Display for BytesPartValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "part {}: ", self.part)?;

        for b in &self.bytes {
            write!(f, "{:02x}", b)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hex32Value {
    pub value: u32,
}

impl fmt::Display for Hex32Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08X}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialCommitHashValue {
    pub raw: u32,
    pub bytes: [u8; 4],
}

impl fmt::Display for PartialCommitHashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.bytes {
            write!(f, "{:02x}", b)?;
        }

        f.write_str("...")
    }
}

pub fn parse_u8(_: AttrKey, raw: u32) -> U8Value {
    U8Value { value: raw as u8 }
}

pub fn parse_u16(_: AttrKey, raw: u32) -> U16Value {
    U16Value { value: raw as u16 }
}

pub fn parse_i16(_: AttrKey, raw: u32) -> I16Value {
    I16Value {
        value: i16::from_le_bytes((raw as u16).to_le_bytes()),
    }
}

pub fn parse_u32(_: AttrKey, raw: u32) -> U32Value {
    U32Value { value: raw }
}

pub fn parse_i32(_: AttrKey, raw: u32) -> I32Value {
    I32Value {
        value: i32::from_le_bytes(raw.to_le_bytes()),
    }
}

pub fn parse_device_id(_: AttrKey, raw: u32) -> DeviceIdValue {
    let did = raw as u8;

    DeviceIdValue {
        raw: did,
        class: (did >> 3) & 0x07,
        sid: did & 0x07,
    }
}

pub fn parse_version(_: AttrKey, raw: u32) -> VersionValue {
    VersionValue {
        caniot: ((raw >> 8) & 0xff) as u8,
        application: (raw & 0xff) as u8,
    }
}

pub fn parse_text(key: AttrKey, raw: u32) -> TextPartValue {
    let bytes = raw.to_le_bytes();

    TextPartValue {
        part: key.part,
        text: String::from_utf8_lossy(&bytes)
            .trim_end_matches('\0')
            .to_string(),
    }
}

pub fn parse_timestamp(_: AttrKey, raw: u32) -> TimestampValue {
    TimestampValue { epoch_seconds: raw }
}

pub fn parse_seconds(_: AttrKey, raw: u32) -> SecondsValue {
    SecondsValue { seconds: raw }
}

pub fn parse_milliseconds(_: AttrKey, raw: u32) -> MillisecondsValue {
    MillisecondsValue { milliseconds: raw }
}

pub fn parse_config_flags(_: AttrKey, raw: u32) -> ConfigFlagsValue {
    ConfigFlagsValue {
        error_response: raw & (1 << 0) != 0,
        telemetry_delay_random: raw & (1 << 1) != 0,
        telemetry_endpoint: ((raw >> 2) & 0x03) as u8,
        telemetry_periodic_enabled: raw & (1 << 4) != 0,
    }
}

pub fn parse_timezone(_: AttrKey, raw: u32) -> TimezoneValue {
    TimezoneValue {
        offset_seconds: i32::from_le_bytes(raw.to_le_bytes()),
    }
}

pub fn parse_location(_: AttrKey, raw: u32) -> LocationValue {
    let b = raw.to_le_bytes();

    LocationValue {
        region: String::from_utf8_lossy(&b[0..2]).to_string(),
        country: String::from_utf8_lossy(&b[2..4]).to_string(),
    }
}

pub fn parse_reset_reason(_: AttrKey, raw: u32) -> ResetReasonValue {
    let reason = match raw {
        0 => "unknown",
        1 => "power-on",
        2 => "watchdog",
        3 => "external",
        4 => "brown-out",
        _ => "invalid",
    };

    ResetReasonValue { reason }
}

pub fn parse_bytes_part(key: AttrKey, raw: u32) -> BytesPartValue {
    BytesPartValue {
        part: key.part,
        bytes: raw.to_le_bytes().to_vec(),
    }
}

pub fn parse_hex32(_: AttrKey, raw: u32) -> Hex32Value {
    Hex32Value { value: raw }
}

pub fn parse_partial_commit_hash(_: AttrKey, raw: u32) -> PartialCommitHashValue {
    PartialCommitHashValue {
        raw,
        bytes: raw.to_le_bytes(),
    }
}
