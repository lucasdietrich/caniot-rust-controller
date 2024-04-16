use tonic::{Request, Response, Result, Status};

use crate::{
    grpcserver::{datetime_to_prost_timestamp, systemtime_to_prost_timestamp},
    shared::SharedHandle,
};

use super::model::{
    caniot_devices_service_server::{CaniotDevicesService, CaniotDevicesServiceServer},
    *,
};

#[derive(Debug)]
pub struct DevicesAPI {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl CaniotDevicesService for DevicesAPI {
    async fn get_list(&self, _request: Request<()>) -> Result<Response<DevicesList>, Status> {
        let (_, devs, _) = self.shared.controller_handle.get_stats().await;

        let devices: Vec<Device> = devs
            .iter()
            .map(|dev| Device {
                did: Some(DeviceId {
                    sid: dev.did.sub_id as u32,
                    cls: dev.did.class as u32,
                }),
                last_seen: dev.last_seen.as_ref().map(datetime_to_prost_timestamp),
                ..Default::default()
            })
            .collect();

        Ok(Response::new(DevicesList { devices }))
    }
}

pub fn get_ng_devices_server(shared: SharedHandle) -> CaniotDevicesServiceServer<DevicesAPI> {
    CaniotDevicesServiceServer::new(DevicesAPI { shared })
}