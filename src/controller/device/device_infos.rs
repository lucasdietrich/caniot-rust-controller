use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::caniot;

use super::{Device, DeviceStats};

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfos {
    pub did: caniot::DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_seen_from_now: Option<u32>, // seconds
    pub controller_attached: bool,
    pub controller_name: Option<String>,
    pub stats: DeviceStats,
    pub measures: Option<caniot::BlcClassTelemetry>,
}

impl Into<DeviceInfos> for &Device {
    fn into(self) -> DeviceInfos {
        DeviceInfos {
            did: self.did,
            last_seen: self.last_seen,
            controller_attached: self.controller.is_some(),
            controller_name: self
                .controller
                .as_ref()
                .and_then(|c| c.wrapper_get_infos().name),
            last_seen_from_now: self.last_seen_from_now(),
            stats: self.stats,
            measures: self.measures,
        }
    }
}
