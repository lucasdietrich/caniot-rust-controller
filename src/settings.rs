use std::str::FromStr;

use chrono::{DateTime, Utc};

const TYPE_STRING: &str = "string";
const TYPE_BOOL: &str = "bool";
const TYPE_INT: &str = "int";
const TYPE_DATETIME: &str = "datetime";

pub trait SettingTrait: FromStr + PartialEq + Clone
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

#[cfg(experimental)]
mod experimental {
    #[derive(Clone)]
    pub struct SettingRef<'a> {
        pub key: &'a str,
        pub type_name: &'static str,
    }

    impl<'a> SettingRef<'a> {
        pub fn new<T>(key: &'a str) -> Self
        where
            T: SettingTrait,
        {
            Self {
                key,
                type_name: T::type_name(),
            }
        }
    }

    pub struct Setting<'a, T: SettingTrait> {
        key: &'a str,
        value: T,
    }

    impl<'a, T: SettingTrait> Setting<'a, T> {
        pub fn new(key: &'a str, value: T) -> Self {
            Self { key, value }
        }

        pub fn key(&self) -> &str {
            &self.key
        }

        pub fn value(&self) -> &T {
            &self.value
        }

        pub fn into_value(self) -> T {
            self.value
        }
    }

    #[derive(Clone)]
    pub struct RawSetting<'a> {
        key_ref: SettingRef<'a>,
        raw_value: String,
    }

    impl<'a> RawSetting<'a> {
        pub fn new<T>(key: &'a str, value: &T) -> Self
        where
            T: SettingTrait,
        {
            Self {
                key_ref: SettingRef::new::<T>(key),
                raw_value: value.as_string(),
            }
        }

        pub fn into_setting<T>(self) -> Option<Setting<'a, T>>
        where
            T: SettingTrait,
        {
            Setting::try_from(self).ok()
        }
    }

    impl<'a, T: SettingTrait> TryFrom<RawSetting<'a>> for Setting<'a, T> {
        type Error = ();

        fn try_from(raw_setting: RawSetting<'a>) -> Result<Self, Self::Error> {
            if raw_setting.key_ref.type_name == T::type_name() {
                match T::try_from_str(&raw_setting.raw_value) {
                    Some(value) => Ok(Setting::new(raw_setting.key_ref.key, value)),
                    None => Err(()),
                }
            } else {
                Err(())
            }
        }
    }
}
