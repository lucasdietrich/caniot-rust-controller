use tonic::{Request, Response, Result, Status};

use super::model::emulation::{
    self as m,
    emulation_service_server::{EmulationService, EmulationServiceServer},
};

use crate::shared::SharedHandle;

#[derive(Debug)]
pub struct NgEmulation {
    pub shared: SharedHandle,
}

fn get_status() -> m::Status {
    m::Status {
        feature_enabled: cfg!(feature = "emu"),
    }
}

#[tonic::async_trait]
impl EmulationService for NgEmulation {
    async fn get(&self, _req: Request<()>) -> Result<Response<m::Status>, Status> {
        Ok(Response::new(get_status()))
    }

    async fn set(&self, req: Request<m::Req>) -> Result<Response<m::Status>, Status> {
        let req = req.into_inner();
        let event = m::EmuRequest::try_from(req.event).unwrap_or_default();
        self.shared
            .controller_handle
            .clone()
            .send_emulation_event(event)
            .await;

        Ok(Response::new(get_status()))
    }
}

pub fn get_ng_emulation_server(shared: SharedHandle) -> EmulationServiceServer<NgEmulation> {
    EmulationServiceServer::new(NgEmulation { shared })
}
