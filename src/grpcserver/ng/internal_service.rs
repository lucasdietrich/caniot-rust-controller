use std::{collections::HashMap, time::SystemTime};

use log::debug;
use tonic::{Request, Response, Status};

use super::model::internal::{
    self as m,
    internal_service_server::{InternalService, InternalServiceServer},
};

use crate::{grpcserver::systemtime_to_prost_timestamp, shared::SharedHandle};

#[derive(Debug)]
pub struct NgInternal {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl InternalService for NgInternal {
    async fn get_settings(&self, _request: Request<()>) -> Result<Response<m::Settings>, Status> {
        let db_lock = self.shared.db.read().await;
        let settings = db_lock.get_settings_store();

        debug!("Reading settings");

        let dark_mode = settings.read("dark_mode").await.unwrap_or(true);
        let debug_mode = settings.read("debug_mode").await.unwrap_or(false);

        Ok(Response::new(m::Settings {
            dark_mode: dark_mode,
            debug_mode: debug_mode,
        }))
    }

    async fn set_settings(
        &self,
        ref request: Request<m::PartialSettings>,
    ) -> Result<Response<m::Settings>, Status> {
        let partial_settings = request.into_inner();
        let db_lock = self.shared.db.read().await;
        let settings = db_lock.get_settings_store();

        debug!("Writing settings");

        let mut success = true;

        if let Some(dark_mode) = partial_settings.dark_mode {
            success &= settings.set("dark_mode", &dark_mode).await.is_ok();
        }

        if let Some(debug_mode) = partial_settings.debug_mode {
            success &= settings.set("debug_mode", &debug_mode).await.is_ok();
        }

        if success {
            self.get_settings(Request::new(())).await
        } else {
            Err(Status::internal("Failed to set settings"))
        }
    }

    async fn reset_settings(&self, _request: Request<()>) -> Result<Response<m::Settings>, Status> {
        let db_lock = self.shared.db.read().await;
        let settings = db_lock.get_settings_store();

        debug!("Resetting settings");

        let mut success = true;
        success &= settings.set("dark_mode", &true).await.is_ok();
        success &= settings.set("debug_mode", &false).await.is_ok();

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
}

pub fn get_ng_internal_server(shared: SharedHandle) -> InternalServiceServer<NgInternal> {
    InternalServiceServer::new(NgInternal { shared })
}
