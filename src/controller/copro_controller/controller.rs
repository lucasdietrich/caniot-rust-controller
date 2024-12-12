use crate::{
    controller::copro_controller::devices::BleDeviceType,
    coprocessor::{CoproHandle, CoproMessage},
};

use chrono::Utc;
use log::info;
use thiserror::Error;

use super::{api_message::CoproApiMessage, devices::BleDevice};

pub struct CoproController {
    handle: CoproHandle,
    devices: Vec<BleDevice>,
}

#[derive(Debug, Error)]
pub enum CoproError {}

impl CoproController {
    pub fn new(handle: CoproHandle) -> Result<CoproController, CoproError> {
        Ok(CoproController {
            handle,
            devices: Vec::new(),
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
                    let device = BleDevice::new(
                        record.ble_addr,
                        BleDeviceType::Xiaomi,
                        record_timestamp,
                        record,
                    );
                    info!("new device: {:?}", device);
                    self.devices.push(device);
                }
            }
        }
    }

    pub async fn handle_api_message(&mut self, message: CoproApiMessage) -> Result<(), CoproError> {
        match message {
            CoproApiMessage::GetDevices { respond_to } => {
                respond_to.send(self.devices.clone()).ok();
            }
        }

        Ok(())
    }
}
