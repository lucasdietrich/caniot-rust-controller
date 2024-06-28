use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum DeviceAlertType {
    Ok,
    Notification,
    Warning,
    Error,
    Inhibitted,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceAlert {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub alert_type: DeviceAlertType,
    pub description: Option<String>,
}

impl DeviceAlert {
    pub fn new(string: &str, alert_type: DeviceAlertType, description: Option<&str>) -> Self {
        Self {
            name: string.to_string(),
            timestamp: Utc::now(),
            alert_type,
            description: description.map(|s| s.to_string()),
        }
    }

    pub fn new_ok(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Ok, None)
    }

    pub fn new_notification(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Notification, None)
    }

    pub fn new_warning(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Warning, None)
    }

    pub fn new_error(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Error, None)
    }

    pub fn new_inhibitted() -> Self {
        Self::new("Actionneur inhibé", DeviceAlertType::Inhibitted, None)
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}
