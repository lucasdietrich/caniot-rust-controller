use std::{
    fmt::{Debug, Display},
    u32,
};

use ble_copro_stream_server::ble::BleAddress;
use chrono::{DateTime, Utc};

use crate::controller::{
    copro_controller::measurements::{
        BatteryTrait, BleMeasurement, EnergyMeterTrait, EnvironmentalTrait, ResetableMinMaxTrait,
        RssiTrait, TimestampedTrait,
    },
    device_filtering::{FilterCriteria, FilterableDevice},
    DeviceAlert,
};

pub const BLE_LOW_BATTERY_THRESHOLD: u8 = 20; // %
pub const BLE_CRITICAL_BATTERY_THRESHOLD: u8 = 5; // %
pub const BLE_TIME_TO_CONSIDER_OFFLINE: u32 = 3600; // seconds
pub const BLE_BAD_RSSI_THRESHOLD: i8 = -90; // dBm

#[derive(Debug, Clone)]
pub enum BleDeviceType {
    Xiaomi,
    LinkyTIC,
}

impl Display for BleDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BleDeviceType::Xiaomi => write!(f, "Xiaomi"),
            BleDeviceType::LinkyTIC => write!(f, "Linky TIC"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub rx_packets: u64,
}

#[derive(Debug, Clone)]
pub struct BleDevice {
    pub device_type: BleDeviceType,
    pub ble_addr: BleAddress,
    pub name: String,
    pub last_seen: DateTime<Utc>,
    pub measurements: BleMeasurement, // Last measurement received
    pub stats: Stats,

    pub battery_mv: Option<u16>,
    pub battery_level: Option<u8>,

    pub rssi: i8, // Last RSSI value received

    ui_display_order: u32,
    location: Option<String>,
}

impl BleDevice {
    pub fn init(
        mac: BleAddress,
        name: String,
        device_type: BleDeviceType,
        measurement_timestamp: DateTime<Utc>,
        rssi: i8,
        battery_mv: Option<u16>,
        battery_level: Option<u8>,
        measurement: impl Into<BleMeasurement>,
        location: Option<String>,
    ) -> Self {
        Self {
            device_type,
            ble_addr: mac,
            last_seen: measurement_timestamp,
            rssi,
            battery_mv,
            battery_level,
            measurements: measurement.into(),
            name,
            stats: Stats { rx_packets: 1 }, // At least one packet received
            ui_display_order: u32::MAX,
            location,
        }
    }

    pub fn reset_measures_minmax(&mut self) {
        self.measurements.reset_minmax();
    }

    pub fn commit_new_environemental_measurement<
        T: EnvironmentalTrait + TimestampedTrait + BatteryTrait + RssiTrait,
    >(
        &mut self,
        timestamp: DateTime<Utc>,
        meas: T,
    ) -> Result<(), T> {
        self.stats.rx_packets += 1;
        self.rssi = meas.rssi();
        self.battery_mv = meas.battery_mv();
        self.battery_level = meas.battery_level();
        self.last_seen = timestamp;
        self.measurements.update_environemental(meas)
    }

    pub fn commit_new_energy_meter_measurement<
        T: EnergyMeterTrait + TimestampedTrait + RssiTrait,
    >(
        &mut self,
        timestamp: DateTime<Utc>,
        meas: T,
    ) -> Result<(), T> {
        self.stats.rx_packets += 1;
        self.last_seen = timestamp;
        self.rssi = meas.rssi();
        self.measurements.update_energy_meter(meas)
    }

    // TODO Remove, calculate in UI
    pub fn last_seen_from_now(&self) -> u32 {
        (Utc::now() - self.last_seen).num_seconds() as u32
    }

    pub fn default_name(device_type: &BleDeviceType, ble_addr: &BleAddress) -> String {
        format!("{} {}", device_type, ble_addr.mac_manufacturer_part())
    }

    pub fn battery_voltage(&self) -> Option<f32> {
        self.battery_mv.map(|v| (v as f32) / 1000.0)
    }

    pub fn battery_level(&self) -> Option<u8> {
        self.battery_level
    }

    pub fn is_low_battery(&self) -> Option<bool> {
        self.battery_level
            .map(|level| level < BLE_LOW_BATTERY_THRESHOLD)
    }

    pub fn is_battery_critical(&self) -> Option<bool> {
        self.battery_level
            .map(|level| level < BLE_CRITICAL_BATTERY_THRESHOLD)
    }

    pub fn is_offline(&self) -> bool {
        self.last_seen_from_now() > BLE_TIME_TO_CONSIDER_OFFLINE
    }

    pub fn is_bad_rssi(&self) -> bool {
        self.rssi < BLE_BAD_RSSI_THRESHOLD
    }

    pub fn get_alert(&self) -> Option<DeviceAlert> {
        if self.is_battery_critical().unwrap_or(false) {
            Some(DeviceAlert::new_error(
                format!(
                    "L'état de la batterie du périphérique \"{}\" est critique: {}% ({} V)",
                    self.name,
                    self.battery_level.unwrap_or(0),
                    self.battery_mv.unwrap_or(0) as f32 / 1000.0
                )
                .as_str(),
            ))
        } else if self.is_low_battery().unwrap_or(false) {
            Some(DeviceAlert::new_warning(
                format!(
                    "L'état de la batterie du périphérique \"{}\" est faible: {}% ({} V)",
                    self.name,
                    self.battery_level.unwrap_or(0),
                    self.battery_mv.unwrap_or(0) as f32 / 1000.0
                )
                .as_str(),
            ))
        } else if self.is_bad_rssi() {
            Some(DeviceAlert::new_notification(
                format!(
                    "Le signal RSSI du périphérique \"{}\" est faible: {} dBm",
                    self.name, self.rssi
                )
                .as_str(),
            ))
        } else if self.is_offline() {
            Some(DeviceAlert::new_notification(
                format!("Le périphérique \"{}\" est hors ligne", self.name).as_str(),
            ))
        } else {
            None
        }
    }

    pub fn set_ui_display_order(&mut self, order: u32) {
        self.ui_display_order = order;
    }

    pub fn get_ui_display_order(&self) -> u32 {
        self.ui_display_order
    }
}

impl FilterableDevice for BleDevice {
    fn get_filter_name(&self) -> String {
        self.name.clone()
    }

    fn get_filter_location(&self) -> Option<String> {
        self.location.clone()
    }

    fn get_default_order(&self) -> u32 {
        self.get_ui_display_order()
    }

    fn get_active_alert(&self) -> Option<DeviceAlert> {
        self.get_alert()
    }

    fn match_criteria(&self, _criteria: &FilterCriteria) -> bool {
        false
    }
}
