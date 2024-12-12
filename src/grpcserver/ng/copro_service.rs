use crate::{
    controller::copro_controller::devices::BleDevice, grpcserver::utc_to_prost_timestamp,
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
}

pub fn get_ng_copro_server(shared: SharedHandle) -> CoproServiceServer<NgCopro> {
    CoproServiceServer::new(NgCopro { shared })
}
