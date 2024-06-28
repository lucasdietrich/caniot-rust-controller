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
    pub string: String,
    pub timestamp: DateTime<Utc>,
    pub alert_type: DeviceAlertType,
}

impl DeviceAlert {
    pub fn new(string: &str, alert_type: DeviceAlertType) -> Self {
        Self {
            string: string.to_string(),
            timestamp: Utc::now(),
            alert_type,
        }
    }

    pub fn new_ok(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Ok)
    }

    pub fn new_notification(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Notification)
    }

    pub fn new_warning(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Warning)
    }

    pub fn new_error(string: &str) -> Self {
        Self::new(string, DeviceAlertType::Error)
    }

    pub fn new_inhibitted() -> Self {
        Self::new("Device is inhibitted", DeviceAlertType::Inhibitted)
    }
}
