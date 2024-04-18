use std::{collections::HashMap, time::SystemTime};

use tonic::{Request, Response, Status};

use super::model::{
    internal_service_server::{InternalService, InternalServiceServer},
    *,
};

use crate::{grpcserver::systemtime_to_prost_timestamp, shared::SharedHandle};

#[derive(Debug)]
pub struct NgInternal {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl InternalService for NgInternal {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut map = HashMap::new();
        map.insert("garage".to_string(), 1);
        map.insert("uuid".to_string(), 2);
        map.insert("second".to_string(), 3);

        let response = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
            timestamp: Some(systemtime_to_prost_timestamp(SystemTime::now())),
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

    // async fn request_telemetry(
    //     &self,
    //     request: Request<TelemetryRequest>,
    // ) -> Result<Response<TelemetryResponse>, Status> {
    //     println!("Got a request: {:?}", request);

    //     let response = TelemetryResponse {
    //         message: format!("Hello!"),
    //     };

    //     Ok(Response::new(response))
    // }
}

pub fn get_ng_internal_server(shared: SharedHandle) -> InternalServiceServer<NgInternal> {
    InternalServiceServer::new(NgInternal { shared })
}
