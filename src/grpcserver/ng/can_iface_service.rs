use tonic::{Request, Response, Result, Status};

use crate::shared::SharedHandle;

use super::model::can_iface::{
    self as m,
    can_iface_service_server::{CanIfaceService, CanIfaceServiceServer},
};

#[derive(Debug)]
pub struct NgCanIface {
    pub shared: SharedHandle,
}

#[tonic::async_trait]
impl CanIfaceService for NgCanIface {
    async fn open(
        &self,
        request: tonic::Request<m::TxCanFrame>,
    ) -> std::result::Result<tonic::Response<m::RxCanFrame>, tonic::Status> {
        // Implement the logic for the `open` method here
        unimplemented!()
    }
}

pub fn get_ng_can_iface_server(shared: SharedHandle) -> CanIfaceServiceServer<NgCanIface> {
    CanIfaceServiceServer::new(NgCanIface { shared })
}
