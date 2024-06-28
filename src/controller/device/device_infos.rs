use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::caniot;

use super::{alert::DeviceAlert, Device, DeviceStats};

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfos {
    pub did: caniot::DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_seen_from_now: Option<u32>, // seconds
    pub controller_attached: bool,
    pub controller_name: Option<String>,
    pub stats: DeviceStats,
    pub measures: Option<caniot::BoardClassTelemetry>,

    // measures
    pub board_temperature: Option<f32>,
    pub outside_temperature: Option<f32>,

    // current alert
    pub active_alert: Option<DeviceAlert>,
}

impl Into<DeviceInfos> for &Device {
    fn into(self) -> DeviceInfos {
        // If controller get the controller infos
        let mut controller_name = None;
        let mut active_alert = None;
        let mut controller_attached = false;
        if let Some(controller) = &self.controller {
            controller_attached = true;
            controller_name = controller.wrapper_get_infos().name;
            active_alert = controller.wrapper_get_alert();
        }

        DeviceInfos {
            did: self.did,
            last_seen: self.last_seen,
            controller_attached: controller_attached,
            controller_name: controller_name,
            last_seen_from_now: self.last_seen_from_now(),
            stats: self.stats,
            measures: self.measures,
            board_temperature: self.measures.and_then(|m| m.get_board_temperature()),
            outside_temperature: self.measures.and_then(|m| m.get_outside_temperature()),
            active_alert,
        }
    }
}
