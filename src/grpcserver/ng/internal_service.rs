use std::{collections::HashMap, time::SystemTime};

use chrono::Utc;
use log::{debug, warn};
use tonic::{Request, Response, Status};

use super::model::internal::{
    self as m,
    internal_service_server::{InternalService, InternalServiceServer},
};

use crate::{
    controller::{CaniotControllerStats, ControllerStats},
    grpcserver::{systemtime_to_prost_timestamp, utc_to_prost_timestamp},
    internal::{
        firmware::{FirmwareBuildInfos, FirmwareInfos},
        software::{SoftwareBuildInfos, SoftwareInfos},
    },
    shared::SharedHandle,
};

#[derive(Debug)]
pub struct NgInternal {
    pub shared: SharedHandle,
}

impl Into<Option<m::SoftwareBuildInfos>> for &SoftwareBuildInfos {
    fn into(self) -> Option<m::SoftwareBuildInfos> {
        if self.is_complete() {
            Some(m::SoftwareBuildInfos {
                version: self.version.to_owned().unwrap(),
                commit: self.get_commit_hash_and_dirty().unwrap(),
                build_date: Some(utc_to_prost_timestamp(&self.build_date.unwrap())),
            })
        } else {
            warn!("SoftwareBuildInfos is not complete");
            None
        }
    }
}

impl Into<m::SoftwareInfos> for &SoftwareInfos {
    fn into(self) -> m::SoftwareInfos {
        m::SoftwareInfos {
            build: (&self.build).into(),
            update_date: None,
            runtime: Some(m::SoftwareRuntimeInfos {
                start_time: Some(utc_to_prost_timestamp(&self.runtime.start_time)),
                system_time: Some(utc_to_prost_timestamp(&Utc::now())),
            }),
        }
    }
}

impl Into<m::FirmwareBuildInfos> for &FirmwareBuildInfos {
    fn into(self) -> m::FirmwareBuildInfos {
        m::FirmwareBuildInfos {
            distro: self.distro.to_owned(),
            distro_version: self.distro_version.to_owned(),
            build_date: self.build_date.map(|ref dt| utc_to_prost_timestamp(dt)),
        }
    }
}

impl Into<m::FirmwareInfos> for &FirmwareInfos {
    fn into(self) -> m::FirmwareInfos {
        m::FirmwareInfos {
            build: Some((&self.build).into()),
        }
    }
}

impl Into<m::ControllerStats> for &ControllerStats {
    fn into(self) -> m::ControllerStats {
        m::ControllerStats {
            iface_rx: self.caniot.iface_rx as u32,
            iface_tx: self.caniot.iface_tx as u32,
            iface_err: self.caniot.iface_err as u32,
            iface_malformed: self.caniot.iface_malformed as u32,
            broadcast_tx: self.caniot.broadcast_tx as u32,
            pq_pushed: self.caniot.pq_pushed as u32,
            pq_answered: self.caniot.pq_answered as u32,
            pq_timeout: self.caniot.pq_timeout as u32,
            pq_duplicate_dropped: self.caniot.pq_duplicate_dropped as u32,
            api_rx: self.core.api_rx as u32,
            loop_runs: self.core.loop_runs as u64,
            can_rx: self.can.rx as u32,
            can_tx: self.can.tx as u32,
            can_err: self.can.err as u32,
            can_unhandled: self.can.unhandled as u32,
        }
    }
}

#[tonic::async_trait]
impl InternalService for NgInternal {
    async fn get_settings(&self, _request: Request<()>) -> Result<Response<m::Settings>, Status> {
        let settings = self.shared.db.get_settings_store();

        debug!("Reading settings");

        let debug_mode = settings.read("debug_mode").await.unwrap_or(false);

        Ok(Response::new(m::Settings {
            debug_mode: debug_mode,
        }))
    }

    async fn set_settings(
        &self,
        ref request: Request<m::PartialSettings>,
    ) -> Result<Response<m::Settings>, Status> {
        let partial_settings = request.into_inner();
        let settings = self.shared.db.get_settings_store();

        debug!("Writing settings");

        let mut success = true;

        if let Some(debug_mode) = partial_settings.debug_mode {
            success &= settings.write("debug_mode", &debug_mode).await.is_ok();
        }

        if success {
            self.get_settings(Request::new(())).await
        } else {
            Err(Status::internal("Failed to set settings"))
        }
    }

    async fn reset_settings(&self, _request: Request<()>) -> Result<Response<m::Settings>, Status> {
        let settings = self.shared.db.get_settings_store();

        debug!("Resetting settings");

        let mut success = settings.delete_all().await.is_ok();
        success &= self
            .shared
            .controller_handle
            .reset_devices_settings()
            .await
            .is_ok();
        if success {
            self.get_settings(Request::new(())).await
        } else {
            Err(Status::internal("Failed to reset settings"))
        }
    }

    async fn hello(
        &self,
        request: Request<m::HelloRequest>,
    ) -> Result<Response<m::HelloResponse>, Status> {
        let mut map = HashMap::new();
        map.insert("garage".to_string(), 1);
        map.insert("uuid".to_string(), 2);
        map.insert("second".to_string(), 3);

        let response = m::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
            timestamp: Some(systemtime_to_prost_timestamp(SystemTime::now())),
            map,
            strings: vec![
                "hello".to_string(),
                "world".to_string(),
                "from".to_string(),
                "Rust".to_string(),
            ],
            bytes: vec![0x01, 0x02, 0x03, 0x04],
        };

        Ok(Response::new(response))
    }

    async fn get_software_infos(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::SoftwareInfos>, Status> {
        Ok(Response::new((&self.shared.software_infos).into()))
    }

    async fn get_firmware_infos(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::FirmwareInfos>, Status> {
        Ok(Response::new((&self.shared.firmware_infos).into()))
    }

    async fn get_infos(&self, _request: Request<()>) -> Result<Response<m::Infos>, Status> {
        Ok(Response::new(m::Infos {
            firmware: Some((&self.shared.firmware_infos).into()),
            software: Some((&self.shared.software_infos).into()),
            controller_stats: Some(
                (&self.shared.controller_handle.get_controller_stats().await).into(),
            ),
        }))
    }

    async fn get_controller_stats(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::ControllerStats>, Status> {
        Ok(Response::new(
            (&self.shared.controller_handle.get_controller_stats().await).into(),
        ))
    }
}

pub fn get_ng_internal_server(shared: SharedHandle) -> InternalServiceServer<NgInternal> {
    InternalServiceServer::new(NgInternal { shared })
}
