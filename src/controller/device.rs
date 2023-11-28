use std::time::{Duration, Instant};

use crate::caniot as ct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub command_tx: usize,
    pub attribute_write: usize,
    pub attribute_read: usize,
    pub err_rx: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Device {
    pub device_id: ct::DeviceId,
    pub last_seen: Option<Instant>,
    pub stats: DeviceStats,

    
}

trait DeviceTrait {
    fn handler_frame(&mut self, frame: ct::Response);
}

impl Device {
    pub fn process_incoming_response(&mut self, frame: &ct::Response) {
        match frame.data {
            ct::ResponseData::Attribute { .. } => {
                self.stats.attribute_read += 1;
            }
            ct::ResponseData::Telemetry { .. } => {
                self.stats.telemetry_rx += 1;
            }
            ct::ResponseData::Error { .. } => {
                self.stats.err_rx += 1;
            }
        }

        self.last_seen = Some(std::time::Instant::now());
    }
}
