use super::settings_types::SettingTrait;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SettingValue {
    String(String),
    Bool(bool),
    I64(i64),
    DateTime(chrono::DateTime<chrono::Utc>),
    NaiveTime(chrono::NaiveTime),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SettingAction {
    name: &'static str,
    value: Option<SettingValue>,
}

impl SettingAction {
    pub fn write(name: &'static str, value: SettingValue) -> Self {
        Self {
            name,
            value: Some(value),
        }
    }

    pub fn delete(name: &'static str) -> Self {
        Self { name, value: None }
    }

    pub fn is_write(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_delete(&self) -> bool {
        self.value.is_none()
    }
}
