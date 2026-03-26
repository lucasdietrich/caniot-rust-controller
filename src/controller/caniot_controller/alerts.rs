use chrono::{DateTime, Utc};

use crate::controller::alert::{DeviceAlertType, SensorAlert};

pub enum AlertSeverity {
    #[allow(dead_code)]
    Debug,
    Info,
    Warning,
    Error,
}

pub enum AlertType {
    DeviceAlert(String),
    #[allow(dead_code)]
    DeviceReboot,
}

pub struct Alert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub timestamp: DateTime<Utc>,
}

impl TryFrom<SensorAlert> for Alert {
    type Error = &'static str;

    fn try_from(alert: SensorAlert) -> Result<Self, Self::Error> {
        Ok(Alert {
            alert_type: AlertType::DeviceAlert(alert.name),
            severity: match alert.alert_type {
                DeviceAlertType::Ok => AlertSeverity::Info,
                DeviceAlertType::Notification => AlertSeverity::Info,
                DeviceAlertType::Warning => AlertSeverity::Warning,
                DeviceAlertType::Error => AlertSeverity::Error,
                DeviceAlertType::Inhibitted => AlertSeverity::Warning,
            },
            timestamp: alert.timestamp,
        })
    }
}
