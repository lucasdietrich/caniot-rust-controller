use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::Instant;

use crate::caniot::{DeviceId, Response, ResponseData};

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub command_tx: usize,
    pub attribute_rx: usize,
    pub attribute_tx: usize,
    pub err_rx: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub did: DeviceId,
    pub last_seen: Option<DateTime<Utc>>,
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

    pub fn mark_last_seen(&mut self) {
        self.last_seen = Some(Utc::now());
    }

    pub fn handle_frame(&mut self, frame: &ResponseData) {
        self.mark_last_seen();

        // update stats
        match frame {
            ResponseData::Telemetry { .. } => self.stats.telemetry_rx += 1,
            ResponseData::Attribute { .. } => self.stats.attribute_rx += 1,
            ResponseData::Error { .. } => self.stats.err_rx += 1,
        }
    }
}

pub trait DeviceTrait {}
