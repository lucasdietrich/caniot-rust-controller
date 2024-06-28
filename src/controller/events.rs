use chrono::{DateTime, Utc};

use super::alert::{DeviceAlert, DeviceAlertType};

pub enum EventSeverity {
    Debug,
    Info,
    Warning,
    Error,
}

pub enum EventType {
    DeviceAlert(String),
    DeviceReboot,
}

pub struct Event {
    pub event_type: EventType,
    pub severity: EventSeverity,
    pub timestamp: DateTime<Utc>,
}

impl TryFrom<DeviceAlert> for Event {
    type Error = &'static str;

    fn try_from(alert: DeviceAlert) -> Result<Self, Self::Error> {
        Ok(Event {
            event_type: EventType::DeviceAlert(alert.name),
            severity: match alert.alert_type {
                DeviceAlertType::Ok => EventSeverity::Info,
                DeviceAlertType::Notification => EventSeverity::Info,
                DeviceAlertType::Warning => EventSeverity::Warning,
                DeviceAlertType::Error => EventSeverity::Error,
                DeviceAlertType::Inhibitted => EventSeverity::Warning,
            },
            timestamp: alert.timestamp,
        })
    }
}
