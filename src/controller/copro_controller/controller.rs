use itertools::Itertools;

use crate::{
    controller::{
        copro_controller::{
            device::BleDeviceType,
            measurements::{
                BatteryTrait, BleEnergyMeterMeasurement, BleEnvironementalMeasurement,
                BleMeasurement, RssiTrait,
            },
        },
        device_filtering::DeviceFilter,
        DeviceAlert,
    },
    coprocessor::{coprocessor::CoproStreamChannelStatus, CoproHandle, CoproMessage},
    impl_display_for_enum,
    utils::{prometheus, PrometheusExporterTrait, PrometheusNoLabel},
};

use ble_copro_stream_server::ble::BleAddress;
use chrono::{DateTime, Utc};
use log::info;
use thiserror::Error;

use super::{api_message::CoproApiMessage, device::BleDevice};

pub struct CoproController {
    handle: CoproHandle,
    devices: Vec<BleDevice>,
    copro_status: CoproStreamChannelStatus,
    stats: CoproControllerStats,
}

#[derive(Debug, Error)]
pub enum CoproError {}

#[derive(Debug, Default, Clone)]
pub struct CoproControllerStats {
    pub rx_packets: u64,
    pub rx_type_xiaomi: u64,
    pub rx_type_linky_tic: u64,
}

impl<'a> PrometheusExporterTrait<'a> for CoproControllerStats {
    type Label = PrometheusNoLabel;

    fn export(&self, _labels: impl AsRef<[&'a Self::Label]>) -> String {
        format!(
            "controller_copro_iface_rx {}\n\
            controller_copro_iface_rx {{type=\"xiaomi\"}} {}\n\
            controller_copro_iface_rx {{type=\"linky_tic\"}} {}\n\
            ",
            self.rx_packets, self.rx_type_xiaomi, self.rx_type_linky_tic,
        )
    }
}

impl CoproController {
    pub fn new(handle: CoproHandle) -> Result<CoproController, CoproError> {
        Ok(CoproController {
            handle,
            devices: Vec::new(),
            copro_status: CoproStreamChannelStatus::Disconnected,
            stats: CoproControllerStats::default(),
        })
    }

    /// Build a new `BleDevice` using (optional) configuration data
    /// from `handle.devices_config` and supplied measurement object.
    fn build_new_device<M: Into<BleMeasurement>>(
        &self,
        addr: BleAddress,
        device_type: BleDeviceType,
        timestamp: DateTime<Utc>,
        rssi: i8,
        battery_mv: Option<u16>,
        battery_level: Option<u8>,
        measurement: M,
    ) -> BleDevice {
        let device_config = self
            .handle
            .devices_config
            .iter()
            .find(|config| config.mac == addr.mac_string());

        // Name / location
        let name = device_config
            .map(|config| config.name.clone())
            .unwrap_or_else(|| BleDevice::default_name(&device_type, &addr));
        let location = device_config.and_then(|config| config.location.clone());

        // Instantiate device
        let mut device = BleDevice::init(
            addr,
            name,
            device_type,
            timestamp,
            rssi,
            battery_mv,
            battery_level,
            measurement,
            location,
        );

        // UI display order (defaults to 0 if not provided)
        let ui_display_order = device_config
            .map(|config| config.ui_display_order)
            .unwrap_or(0);
        device.set_ui_display_order(ui_display_order);

        device
    }

    pub async fn poll_message(&mut self) -> Option<CoproMessage> {
        self.handle.receiver.recv().await
    }

    pub async fn handle_message(&mut self, message: CoproMessage) {
        match message {
            CoproMessage::Xiaomi(record) => {
                info!("ble xiaomi {}", record);
                self.stats.rx_packets += 1;
                self.stats.rx_type_xiaomi += 1;

                let record_timestamp = record.timestamp.to_utc().unwrap_or(Utc::now());

                if let Some(device) = self
                    .devices
                    .iter_mut()
                    .find(|d| d.ble_addr == record.ble_addr)
                {
                    let _ = device.commit_new_environemental_measurement(record_timestamp, record);
                } else {
                    let device = self.build_new_device(
                        record.ble_addr,
                        BleDeviceType::Xiaomi,
                        record_timestamp,
                        record.rssi(),
                        record.battery_mv(),
                        record.battery_level(),
                        BleEnvironementalMeasurement::init(record),
                    );
                    info!("new device: {:?}", device);
                    self.devices.push(device);
                }
            }
            CoproMessage::LinkyTic(record) => {
                info!("ble linky tic {}", record);
                self.stats.rx_packets += 1;
                self.stats.rx_type_linky_tic += 1;

                let record_timestamp = record.timestamp.to_utc().unwrap_or(Utc::now());

                if let Some(device) = self
                    .devices
                    .iter_mut()
                    .find(|d| d.ble_addr == record.ble_addr)
                {
                    let _ = device.commit_new_energy_meter_measurement(record_timestamp, record);
                } else {
                    let device = self.build_new_device(
                        record.ble_addr,
                        BleDeviceType::LinkyTIC,
                        record_timestamp,
                        record.rssi(),
                        None,
                        None,
                        BleEnergyMeterMeasurement::init(record),
                    );
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

    // Return a list of devices with given filter
    fn get_devices(&self, filter: DeviceFilter) -> Vec<BleDevice> {
        let filter_function = filter.get_filter_function::<BleDevice>();
        let sort_function = filter.get_sort_function::<BleDevice>();
        self.devices
            .iter()
            .filter(|device| filter_function(device))
            .sorted_by(|a, b| sort_function(a, b))
            .cloned()
            .collect()
    }

    pub async fn handle_api_message(&mut self, message: CoproApiMessage) -> Result<(), CoproError> {
        match message {
            CoproApiMessage::GetDevices { respond_to, filter } => {
                let devices = self.get_devices(filter);
                respond_to.send(devices).ok();
            }
            CoproApiMessage::GetAlert { respond_to } => {
                respond_to.send(self.get_controller_alert()).ok();
            }
            CoproApiMessage::GetStats { respond_to } => {
                respond_to.send(self.stats.clone()).ok();
            }
            CoproApiMessage::ResetDevicesMeasuresStats => {
                for device in self.devices.iter_mut() {
                    device.reset_measures_minmax();
                }
            }
        }

        Ok(())
    }
}
