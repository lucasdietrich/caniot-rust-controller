use crate::{
    controller::copro_controller::device::BleDevice, grpcserver::utc_to_prost_timestamp,
    shared::SharedHandle,
};

use super::model::copro::{
    self as m,
    copro_service_server::{CoproService, CoproServiceServer},
};

impl Into<m::CoproDevice> for &BleDevice {
    fn into(self) -> m::CoproDevice {
        m::CoproDevice {
            mac: self.ble_addr.mac_string(),
            name: self.name.to_owned(),
            r#type: self.device_type.to_string(),
            last_seen: Some(utc_to_prost_timestamp(&self.last_seen)),
            last_seen_from_now: Some(self.last_seen_from_now()),
            is_seen: true,
            rssi: Some(self.last_measurement.rssi() as i32),
            temperature: self.last_measurement.temperature(),
            humidity: self.last_measurement.humidity(),
            battery_level: self.last_measurement.battery_level().map(|v| v as i32),
            battery_voltage: self.last_measurement.battery_voltage(),
            stats: Some(m::CoproDeviceStats {
                rx: self.stats.rx_packets,
            }),
            active_alert: self.get_alert().as_ref().map(|a| a.into()),
            temperature_min: self.measures.get_temperature_monitor().get_min().cloned(),
            temperature_max: self.measures.get_temperature_monitor().get_max().cloned(),
            humidity_min: self.measures.get_humidity_monitor().get_min().cloned(),
            humidity_max: self.measures.get_humidity_monitor().get_max().cloned(),
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
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<m::CoproDevicesList>, tonic::Status> {
        let devices: Vec<m::CoproDevice> = self
            .shared
            .controller_handle
            .get_copro_devices_list()
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
}

pub fn get_ng_copro_server(shared: SharedHandle) -> CoproServiceServer<NgCopro> {
    CoproServiceServer::new(NgCopro { shared })
}
