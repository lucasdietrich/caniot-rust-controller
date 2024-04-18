use tonic::{Request, Response, Result, Status};

use crate::{grpcserver::datetime_to_prost_timestamp, shared::SharedHandle};

use super::model_heaters::{
    self as m,
    heaters_service_server::{HeatersService, HeatersServiceServer},
};

#[derive(Debug)]
pub struct NgHeaters {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl HeatersService for NgHeaters {
    async fn get_state(&self, req: Request<()>) -> Result<Response<m::Status>, Status> {
        todo!()
    }

    async fn set_state(&self, req: Request<m::Command>) -> Result<Response<m::Status>, Status> {
        todo!()
    }
}

pub fn get_ng_heaters_server(shared: SharedHandle) -> HeatersServiceServer<NgHeaters> {
    HeatersServiceServer::new(NgHeaters { shared })
}
