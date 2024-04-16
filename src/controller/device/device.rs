use serde::Serialize;
use std::time::Instant;

use crate::caniot::DeviceId;

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

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub did: DeviceId,
    pub last_seen: Option<Instant>,
    pub stats: DeviceStats,
}

impl Device {
    pub fn new(did: DeviceId) -> Self {
        Self {
            did,
            last_seen: None,
            stats: DeviceStats::default(),
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Some(Instant::now());
    }
}

pub trait DeviceTrait {}
