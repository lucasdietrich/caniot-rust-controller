use std::str::FromStr;

use chrono::{DateTime, NaiveTime, Utc};

const TYPE_STRING: &str = "string";
const TYPE_BOOL: &str = "bool";
const TYPE_INT: &str = "int";
const TYPE_U32: &str = "u32";
const TYPE_DATETIME: &str = "datetime";
const TYPE_NAIVETIME: &str = "naivetime";

pub trait SettingTrait: FromStr + PartialEq + Clone + Default
where
    Self: Sized,
{
    fn type_name() -> &'static str;
    fn as_string(&self) -> String;
    fn try_from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl SettingTrait for String {
    fn type_name() -> &'static str {
        TYPE_STRING
    }

    fn as_string(&self) -> String {
        self.clone()
    }
}

impl SettingTrait for bool {
    fn type_name() -> &'static str {
        TYPE_BOOL
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl SettingTrait for i64 {
    fn type_name() -> &'static str {
        TYPE_INT
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl SettingTrait for u32 {
    fn type_name() -> &'static str {
        TYPE_U32
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl SettingTrait for DateTime<Utc> {
    fn type_name() -> &'static str {
        TYPE_DATETIME
    }

    fn as_string(&self) -> String {
        self.to_rfc3339()
    }

    fn try_from_str(s: &str) -> Option<Self> {
        DateTime::parse_from_rfc3339(s)
            .map_err(|e| log::error!("parse_from_rfc3339: {}", e))
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

impl SettingTrait for NaiveTime {
    fn type_name() -> &'static str {
        TYPE_NAIVETIME
    }

    fn as_string(&self) -> String {
        self.to_string()
    }

    fn try_from_str(s: &str) -> Option<Self> {
        NaiveTime::parse_from_str(s, "%H:%M:%S")
            .map_err(|e| log::error!("parse_from_str: {}", e))
            .ok()
    }
}
