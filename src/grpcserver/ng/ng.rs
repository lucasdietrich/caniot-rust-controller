use std::{collections::HashMap, time::Instant};

use log::info;
use tonic::{
    transport::{Error as GrpcError, Server},
    Code, Request, Response, Status,
};

use model::caniot_controller_server::{CaniotController, CaniotControllerServer};
use model::*;

pub mod model {
    tonic::include_proto!("ng");
}

use crate::shared::SharedHandle;

#[derive(Debug)]
pub struct NgCaniotController {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl CaniotController for NgCaniotController {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        
        let mut map = HashMap::new();
        map.insert("garage".to_string(), 1);
        map.insert("uuid".to_string(), 2);
        map.insert("second".to_string(), 3);

        let response = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
            timestamp: Some(prost_types::Timestamp {
                seconds: now.as_secs() as i64,
                    nanos: now.subsec_nanos() as i32,
            }),
            map: map,
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

    async fn hello_empty(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn request_telemetry(
        &self,
        request: Request<TelemetryRequest>,
    ) -> Result<Response<TelemetryResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = TelemetryResponse {
            message: format!("Hello!"),
        };

        Ok(Response::new(response))
    }
}

pub fn get_ng_caniot_controller(
    shared: SharedHandle,
) -> CaniotControllerServer<NgCaniotController> {
    CaniotControllerServer::new(NgCaniotController { shared })
}
