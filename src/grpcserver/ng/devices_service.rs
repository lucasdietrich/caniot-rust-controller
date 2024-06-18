use tonic::{Request, Response, Result, Status};

use crate::{
    caniot,
    controller::{
        auto_attach::{DEVICE_GARAGE_DID, DEVICE_HEATERS_DID, DEVICE_OUTDOOR_ALARM_DID},
        DeviceInfos,
    },
    grpcserver::datetime_to_prost_timestamp,
    shared::SharedHandle,
};

use super::model as m;
use super::model::caniot_devices_service_server::{
    CaniotDevicesService, CaniotDevicesServiceServer,
};

#[derive(Debug)]
pub struct NgDevices {
    pub shared: SharedHandle,
}

impl NgDevices {
    async fn get_device_by_did(
        &self,
        did: caniot::DeviceId,
    ) -> Result<Response<m::Device>, Status> {
        if let Some(ref infos) = self.shared.controller_handle.get_device_infos(did).await {
            Ok(Response::new(infos.into()))
        } else {
            Err(Status::not_found("Device not found"))
        }
    }
}

impl Into<m::DeviceId> for caniot::DeviceId {
    fn into(self) -> m::DeviceId {
        m::DeviceId {
            did: self.to_u8() as u32,
        }
    }
}

impl Into<m::DeviceIdInfos> for caniot::DeviceId {
    fn into(self) -> m::DeviceIdInfos {
        m::DeviceIdInfos {
            did: self.to_u8() as u32,
            sid: self.sub_id as u32,
            cls: self.class as u32,
        }
    }
}

impl Into<caniot::DeviceId> for m::DeviceId {
    fn into(self) -> caniot::DeviceId {
        caniot::DeviceId::try_from_u8(self.did as u8).unwrap()
    }
}

impl Into<m::Class0Telemetry> for caniot::class0::Telemetry {
    fn into(self) -> m::Class0Telemetry {
        m::Class0Telemetry {
            in1: self.in1,
            in2: self.in2,
            in3: self.in3,
            in4: self.in4,
            oc1: self.oc1,
            oc2: self.oc2,
            rl1: self.rl1,
            rl2: self.rl2,
            int_temp: self.temp_in.to_celsius(),
            ext_temp0: self.temp_out[0].to_celsius(),
            ext_temp1: self.temp_out[1].to_celsius(),
            ext_temp2: self.temp_out[2].to_celsius(),
        }
    }
}

impl Into<m::Class1Telemetry> for caniot::class1::Telemetry {
    fn into(self) -> m::Class1Telemetry {
        m::Class1Telemetry {
            ios: self.ios.to_vec(),
            int_temp: self.temp_in.to_celsius(),
            ext_temp0: self.temp_out[0].to_celsius(),
            ext_temp1: self.temp_out[1].to_celsius(),
            ext_temp2: self.temp_out[2].to_celsius(),
        }
    }
}

impl Into<m::device::Measures> for caniot::classes::BoardClassTelemetry {
    fn into(self) -> m::device::Measures {
        match self {
            caniot::BoardClassTelemetry::Class0(t) => m::device::Measures::Class0(t.into()),
            caniot::BoardClassTelemetry::Class1(t) => m::device::Measures::Class1(t.into()),
        }
    }
}

impl Into<m::Device> for &DeviceInfos {
    fn into(self) -> m::Device {
        m::Device {
            did: Some(self.did.into()),
            last_seen: self.last_seen.as_ref().map(datetime_to_prost_timestamp),
            last_seen_from_now: self.last_seen_from_now,
            controller_attached: self.controller_attached,
            controller_name: self.controller_name.clone(),
            stats: Some(m::DeviceStats {
                rx: self.stats.rx as u32,
                tx: self.stats.tx as u32,
                telemetry_rx: self.stats.telemetry_rx as u32,
                telemetry_tx: self.stats.telemetry_tx as u32,
                command_tx: self.stats.command_tx as u32,
                err_rx: self.stats.err_rx as u32,
                attribute_rx: self.stats.attribute_rx as u32,
                attribute_tx: self.stats.attribute_tx as u32,
            }),
            board_temp: self.board_temperature,
            measures: self.measures.map(|m| m.into()),
            ..Default::default()
        }
    }
}

#[tonic::async_trait]
impl CaniotDevicesService for NgDevices {
    async fn get_list(&self, _request: Request<()>) -> Result<Response<m::DevicesList>, Status> {
        let devices: Vec<m::Device> = self
            .shared
            .controller_handle
            .get_devices_infos_list()
            .await
            .iter()
            .map(|dev| dev.into())
            .collect();

        Ok(Response::new(m::DevicesList { devices }))
    }

    async fn get(&self, request: Request<m::DeviceId>) -> Result<Response<m::Device>, Status> {
        let did: caniot::DeviceId = request.into_inner().into();
        if let Some(ref infos) = self.shared.controller_handle.get_device_infos(did).await {
            Ok(Response::new(infos.into()))
        } else {
            Err(Status::not_found("Device not found"))
        }
    }

    async fn get_heaters_device(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::Device>, Status> {
        self.get_device_by_did(caniot::DeviceId::from_u8(DEVICE_HEATERS_DID))
            .await
    }

    async fn get_garage_device(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::Device>, Status> {
        self.get_device_by_did(caniot::DeviceId::from_u8(DEVICE_GARAGE_DID))
            .await
    }

    async fn get_outdoor_alarm_device(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::Device>, Status> {
        self.get_device_by_did(caniot::DeviceId::from_u8(DEVICE_OUTDOOR_ALARM_DID))
            .await
    }
}

pub fn get_ng_devices_server(shared: SharedHandle) -> CaniotDevicesServiceServer<NgDevices> {
    CaniotDevicesServiceServer::new(NgDevices { shared })
}
