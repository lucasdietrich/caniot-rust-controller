use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::caniot;

use super::{Device, DeviceStats};

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfos {
    pub did: caniot::DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_seen_from_now: Option<u32>, // seconds
    pub stats: DeviceStats,
    pub measures: Option<caniot::BlcClassTelemetry>,
}

impl Into<DeviceInfos> for &Device {
    fn into(self) -> DeviceInfos {
        DeviceInfos {
            did: self.did,
            last_seen: self.last_seen,
            last_seen_from_now: self.last_seen_from_now(),
            stats: self.stats,
            measures: self.measures,
        }
    }
}
