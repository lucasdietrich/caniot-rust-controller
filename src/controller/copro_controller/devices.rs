use std::fmt::{Debug, Display};

use ble_copro_stream_server::ble::BleAddress;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum BleDeviceType {
    Xiaomi,
}

impl Display for BleDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BleDeviceType::Xiaomi => write!(f, "Xiaomi"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BleMeasurement {
    pub timestamp: Option<DateTime<Utc>>,
    pub rssi: i8,
    pub battery_mv: Option<u16>,
    pub battery_level: Option<u8>,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
}

impl BleMeasurement {
    pub fn new(
        timestamp: Option<DateTime<Utc>>,
        rssi: i8,
        battery_mv: Option<u16>,
        battery_level: Option<u8>,
        temperature: Option<f32>,
        humidity: Option<f32>,
    ) -> Self {
        Self {
            timestamp,
            rssi,
            battery_mv,
            battery_level,
            temperature,
            humidity,
        }
    }

    pub fn battery_voltage(&self) -> Option<f32> {
        self.battery_mv.map(|v| (v as f32) / 1000.0)
    }

    pub fn rssi(&self) -> i8 {
        self.rssi
    }

    pub fn battery_level(&self) -> Option<u8> {
        self.battery_level
    }

    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    pub fn humidity(&self) -> Option<f32> {
        self.humidity
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
    pub last_seen: DateTime<Utc>,
    pub last_measurement: BleMeasurement,
    pub name: String,
    pub stats: Stats,
}

impl BleDevice {
    pub fn new(
        mac: BleAddress,
        device_type: BleDeviceType,
        measurement_timestamp: DateTime<Utc>,
        measurement: impl Into<BleMeasurement>,
    ) -> Self {
        let default_name = BleDevice::default_name(&device_type, &mac);
        Self {
            device_type,
            ble_addr: mac,
            last_seen: measurement_timestamp,
            last_measurement: measurement.into(),
            name: default_name,
            stats: Stats { rx_packets: 0 },
        }
    }

    pub fn handle_received_frame(
        &mut self,
        measurement_timestamp: DateTime<Utc>,
        measurement: impl Into<BleMeasurement>,
    ) {
        self.stats.rx_packets += 1;
        self.last_seen = measurement_timestamp;
        self.last_measurement = measurement.into();
    }

    // TODO Remove, calculate in UI
    pub fn last_seen_from_now(&self) -> u32 {
        (Utc::now() - self.last_seen).num_seconds() as u32
    }

    pub fn default_name(device_type: &BleDeviceType, ble_addr: &BleAddress) -> String {
        format!("{} {}", device_type, ble_addr.mac_manufacturer_part())
    }
}
