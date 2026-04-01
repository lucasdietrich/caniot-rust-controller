use crate::{
    controller::{
        copro_controller::{
            controller::{BleDevice, BleDevicePairingState},
            measurements::{
                BleMeasurement, EnergyMeterMinMaxTrait, EnergyMeterTrait, EnvironmentalMinMaxTrait,
                EnvironmentalTrait,
            },
            sensor::BleSensor,
        },
        filtering::SensorFilter,
    },
    grpcserver::utc_to_prost_timestamp,
    shared::SharedHandle,
};

use super::model::copro::{
    self as m,
    copro_service_server::{CoproService, CoproServiceServer},
};

impl Into<m::copro_device::Measurements> for &BleMeasurement {
    fn into(self) -> m::copro_device::Measurements {
        match self {
            BleMeasurement::Environemental(meas) => {
                m::copro_device::Measurements::Environemental(m::EnvironementalMeasurement {
                    temperature: meas.temperature(),
                    humidity: meas.humidity(),
                    temperature_min: meas.temperature_min(),
                    temperature_max: meas.temperature_max(),
                    humidity_min: meas.humidity_min(),
                    humidity_max: meas.humidity_max(),
                })
            }
            BleMeasurement::EnergyMeter(meas) => {
                m::copro_device::Measurements::EnergyMeter(m::EnergyMeterMeasurement {
                    current: meas.current(),
                    power: meas.power(),
                    energy: meas.energy(),
                    power_min: meas.power_min(),
                    power_max: meas.power_max(),
                    current_min: meas.current_min(),
                    current_max: meas.current_max(),
                })
            }
        }
    }
}

impl From<&BleDevice> for m::BleDevice {
    fn from(d: &BleDevice) -> Self {
        let (pairing_state, pairing_code) = match d.pairing {
            BleDevicePairingState::None => (m::BleDevicePairingState::BlePairingNone as i32, None),
            BleDevicePairingState::Pending { code } => (
                m::BleDevicePairingState::BlePairingPending as i32,
                Some(code),
            ),
            BleDevicePairingState::Succeeded => {
                (m::BleDevicePairingState::BlePairingSucceeded as i32, None)
            }
            BleDevicePairingState::Failed => {
                (m::BleDevicePairingState::BlePairingFailed as i32, None)
            }
        };
        m::BleDevice {
            mac: d.addr.mac_string(),
            connected: d.connected,
            pairing_state,
            pairing_code,
            stats: Some(m::BleDeviceStats {
                connection_events: d.stats.connection_events,
                pairing_events: d.stats.pairing_events,
                commands_received: d.stats.commands_received,
            }),
            last_seen: Some(utc_to_prost_timestamp(&d.last_seen)),
        }
    }
}

impl Into<m::CoproDevice> for &BleSensor {
    fn into(self) -> m::CoproDevice {
        m::CoproDevice {
            mac: self.ble_addr.mac_string(),
            name: self.name.to_owned(),
            r#type: self.sensor_type.to_string(),
            last_seen: Some(utc_to_prost_timestamp(&self.last_seen)),
            last_seen_from_now: Some(self.last_seen_from_now()),
            is_seen: true,
            rssi: Some(self.rssi as i32),
            battery_level: self.battery_level().map(|v| v as i32),
            battery_voltage: self.battery_voltage(),
            stats: Some(m::CoproDeviceStats {
                rx: self.stats.rx_packets,
            }),
            active_alert: self.get_alert().as_ref().map(|a| a.into()),
            measurements: Some((&self.measurements).into()),
        }
    }
}

#[derive(Debug)]
pub struct NgCopro {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl CoproService for NgCopro {
    async fn get_list(
        &self,
        ref req: tonic::Request<m::GetListParams>,
    ) -> Result<tonic::Response<m::CoproDevicesList>, tonic::Status> {
        let filter = req
            .into_inner()
            .filter
            .map(|f| match f {
                m::get_list_params::Filter::All(()) => SensorFilter::All,
                m::get_list_params::Filter::Name(name) => SensorFilter::ByName(name),
            })
            .unwrap_or(SensorFilter::All);

        let devices: Vec<m::CoproDevice> = self
            .shared
            .controller_handle
            .get_copro_devices_by_filter(filter)
            .await
            .into_iter()
            .map(|ref dev| dev.into())
            .collect();

        Ok(tonic::Response::new(m::CoproDevicesList { devices }))
    }

    async fn get_copro_alert(
        &self,
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<m::CoproAlert>, tonic::Status> {
        let alert = self.shared.controller_handle.get_copro_alert().await;

        Ok(tonic::Response::new(m::CoproAlert {
            active_alert: alert.as_ref().map(|a| a.into()),
            ..Default::default()
        }))
    }

    async fn get_ble_devices(
        &self,
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<m::BleDevicesList>, tonic::Status> {
        let devices = self.shared.controller_handle.get_ble_devices_state().await;
        Ok(tonic::Response::new(m::BleDevicesList {
            devices: devices.iter().map(|d| d.into()).collect(),
        }))
    }

    async fn ble_remove_bonds(
        &self,
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        self.shared.controller_handle.ble_remove_bonds().await;
        Ok(tonic::Response::new(()))
    }

    async fn ble_enable_pairing_adv(
        &self,
        req: tonic::Request<m::BlePairingAdvParams>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let duration_s = req.into_inner().duration_s;
        self.shared
            .controller_handle
            .ble_enable_pairing_adv(duration_s)
            .await;
        Ok(tonic::Response::new(()))
    }

    async fn get_pairing_adv_state(
        &self,
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<m::PairingAdvState>, tonic::Status> {
        let (active, duration_s) = self.shared.controller_handle.get_pairing_adv_state().await;
        Ok(tonic::Response::new(m::PairingAdvState {
            active,
            duration_s,
        }))
    }

    async fn get_ble_firmware_version(
        &self,
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<m::BleFirmwareVersion>, tonic::Status> {
        let version = self
            .shared
            .controller_handle
            .get_copro_firmware_version()
            .await;
        Ok(tonic::Response::new(m::BleFirmwareVersion { version }))
    }
}

pub fn get_ng_copro_server(shared: SharedHandle) -> CoproServiceServer<NgCopro> {
    CoproServiceServer::new(NgCopro { shared })
}
