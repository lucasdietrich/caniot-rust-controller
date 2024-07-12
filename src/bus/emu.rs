use std::time::{Duration, Instant};

use crate::{
    caniot::{
        self,
        emu::{self, emu_pool2_realistic_add_devices_to_iface},
    },
    grpcserver::EmuRequest,
};

use socketcan::CanDataFrame;
use tokio::time::sleep;

use super::{CanConfig, CanInterfaceError, CanInterfaceTrait, CanStats};

pub struct CanInterface {
    stats: CanStats,
    devices: Vec<emu::Device>,
    to_recv_msgq: Vec<CanDataFrame>,
}

impl CanInterface {
    pub fn add_device(&mut self, device: emu::Device) {
        self.devices.push(device);
    }

    fn send_emu_request(&mut self, event: EmuRequest) {
        let now = Instant::now();
        for device in self.devices.iter_mut() {
            device.handle_emu_request(event);

            // Process the device immediately after having send the emulated event
            // If a response is generated, it will be added to the to_recv_msgq
            if let Some(req) = device.process(None, &now) {
                self.to_recv_msgq.push(req.into());
            }
        }
    }
}

#[async_trait]
impl CanInterfaceTrait for CanInterface {
    async fn new(_config: &CanConfig) -> Result<Self, CanInterfaceError> {
        warn!("Using emulated CAN interface");

        let mut iface = Self {
            stats: CanStats::default(),
            devices: vec![],
            to_recv_msgq: Vec::new(),
        };

        emu_pool2_realistic_add_devices_to_iface(&mut iface);

        Ok(iface)
    }

    async fn send(&mut self, frame: CanDataFrame) -> Result<(), CanInterfaceError> {
        self.stats.tx += 1;

        let now = Instant::now();
        if let Ok(caniot_query) = caniot::Request::try_from(frame) {
            for device in self.devices.iter_mut() {
                if device.did == caniot_query.device_id {
                    if let Some(caniot_response) = device.process(Some(&caniot_query.data), &now) {
                        self.to_recv_msgq.push(caniot_response.into());
                    }
                }
            }
        } else {
            warn!("Invalid CAN query frame")
        }

        Ok(())
    }

    async fn recv_poll(&mut self) -> Option<CanDataFrame> {
        if let Some(frame) = self.to_recv_msgq.pop() {
            return Some(frame);
        }

        loop {
            let mut next_telemetry: Option<Duration> = None;

            let now = Instant::now();

            for device in self.devices.iter_mut() {
                if let Some(caniot_response) = device.process(None, &now) {
                    return Some(caniot_response.into());
                }

                let device_next_telemetry = device.get_time_to_next_device_process(&now);
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

    fn get_stats(&self) -> CanStats {
        self.stats
    }

    fn ioctl(&mut self, cmd: u32, arg: u32) -> Result<(), CanInterfaceError> {
        match cmd {
            super::CAN_IOCTL_SEND_EMU_EVENT => {
                let event = EmuRequest::try_from(arg as i32).unwrap_or_default();
                self.send_emu_request(event)
            }
            cmd => {
                error!("Unsupported ioctl command {}", cmd);
            }
        }

        Ok(())
    }
}
