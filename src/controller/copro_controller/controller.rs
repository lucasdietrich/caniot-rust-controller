use crate::{
    controller::{copro_controller::device::BleDeviceType, DeviceAlert},
    coprocessor::{coprocessor::CoproStreamChannelStatus, CoproHandle, CoproMessage},
};

use chrono::Utc;
use log::info;
use thiserror::Error;

use super::{api_message::CoproApiMessage, device::BleDevice};

pub struct CoproController {
    handle: CoproHandle,
    devices: Vec<BleDevice>,
    copro_status: CoproStreamChannelStatus,
}

#[derive(Debug, Error)]
pub enum CoproError {}

impl CoproController {
    pub fn new(handle: CoproHandle) -> Result<CoproController, CoproError> {
        Ok(CoproController {
            handle,
            devices: Vec::new(),
            copro_status: CoproStreamChannelStatus::Disconnected,
        })
    }

    pub async fn poll_message(&mut self) -> Option<CoproMessage> {
        self.handle.receiver.recv().await
    }

    pub async fn handle_message(&mut self, message: CoproMessage) {
        match message {
            CoproMessage::XiaomiRecord(record) => {
                info!("ble xiaomi {}", record);

                let record_timestamp = record.timestamp.to_utc().unwrap_or(Utc::now());

                if let Some(device) = self
                    .devices
                    .iter_mut()
                    .find(|d| d.ble_addr == record.ble_addr)
                {
                    let _ = device.handle_received_frame(record_timestamp, record);
                } else {
                    let device_config = self
                        .handle
                        .devices_config
                        .iter()
                        .find(|config| config.mac == record.ble_addr.mac_string());

                    // Set name for the device
                    let name = device_config
                        .map(|config| config.name.clone())
                        .unwrap_or_else(|| {
                            BleDevice::default_name(&BleDeviceType::Xiaomi, &record.ble_addr)
                        });

                    let mut device = BleDevice::new(
                        record.ble_addr,
                        name,
                        BleDeviceType::Xiaomi,
                        record_timestamp,
                        record,
                    );

                    // Set display order for the device
                    let ui_display_order = device_config
                        .map(|config| config.ui_display_order)
                        .unwrap_or(0);
                    device.set_ui_display_order(ui_display_order);

                    info!("new device: {:?}", device);
                    self.devices.push(device);
                }
            }
            CoproMessage::Status(status) => {
                info!("Coprocessor status changed: {:?}", status);
                self.copro_status = status;
            }
        }
    }

    fn get_controller_alert(&self) -> Option<DeviceAlert> {
        match self.copro_status {
            CoproStreamChannelStatus::Error(ref msg) => Some(DeviceAlert::new_error(msg)),
            CoproStreamChannelStatus::Disconnected => Some(DeviceAlert::new_warning(
                "BLE Coprocessor dongle undetected",
            )),
            CoproStreamChannelStatus::Connected => {
                Some(DeviceAlert::new_ok("BLE Coprocessor dongle connected"))
            }
        }
    }

    pub async fn handle_api_message(&mut self, message: CoproApiMessage) -> Result<(), CoproError> {
        match message {
            CoproApiMessage::GetDevices { respond_to } => {
                let mut devices = self.devices.clone();
                devices.sort_by_key(|dev| dev.get_ui_display_order());
                respond_to.send(devices).ok();
            }
            CoproApiMessage::GetAlert { respond_to } => {
                respond_to.send(self.get_controller_alert()).ok();
            }
        }

        Ok(())
    }
}
