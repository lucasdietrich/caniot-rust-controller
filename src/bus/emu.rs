use std::time::Duration;

use crate::caniot::{self, emu};

use super::can::{CanConfig, CanInterfaceError, CanStats};
use socketcan::CanFrame;
use tokio::time::sleep;

pub struct CanInterface {
    stats: CanStats,
    devices: Vec<emu::Device>,
    to_recv_msgq: Vec<CanFrame>,
}

impl CanInterface {
    pub async fn new(_config: &CanConfig) -> Result<Self, CanInterfaceError> {
        warn!("Using emulated CAN interface");

        Ok(Self {
            stats: CanStats::default(),
            devices: vec![],
            to_recv_msgq: Vec::new(),
        })
    }

    pub fn add_device(&mut self, device: emu::Device) {
        self.devices.push(device);
    }

    pub async fn send(&mut self, frame: CanFrame) -> Result<(), CanInterfaceError> {
        self.stats.tx += 1;

        if let Ok(caniot_query) = caniot::Request::try_from(frame) {
            for device in self.devices.iter_mut() {
                if device.did == caniot_query.device_id {
                    if let Some(caniot_response) = device.process(Some(&caniot_query.data)) {
                        self.to_recv_msgq.push(caniot_response.into());
                    }
                }
            }
        } else {
            warn!("Invalid CAN query frame")
        }

        Ok(())
    }

    pub async fn recv_poll(&mut self) -> Option<CanFrame> {
        if let Some(frame) = self.to_recv_msgq.pop() {
            return Some(frame);
        }

        loop {
            let mut next_telemetry: Option<Duration> = None;

            for device in self.devices.iter_mut() {
                if let Some(caniot_response) = device.process(None) {
                    return Some(caniot_response.into());
                }

                let device_next_telemetry = device.get_time_to_next_device_process();
                if let Some(device_next_telemetry) = device_next_telemetry {
                    if device_next_telemetry <= next_telemetry.unwrap_or(device_next_telemetry) {
                        next_telemetry = Some(device_next_telemetry);
                    }
                }
            }

            if let Some(next_telemetry) = next_telemetry {
                sleep(next_telemetry).await;
            }
        }
    }

    pub fn get_stats(&self) -> CanStats {
        self.stats
    }
}
