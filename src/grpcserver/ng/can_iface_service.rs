// Help for implementing the streams RPC: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md#creating-the-server

use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Result, Status, Streaming};

use crate::shared::SharedHandle;

use super::model::can_iface::{
    self as m,
    can_iface_service_server::{CanIfaceService, CanIfaceServiceServer},
};

#[derive(Debug)]
pub struct NgCanIface {
    pub shared: SharedHandle,
}

type RxStream = ReceiverStream<Result<m::RxCanFrame, Status>>;

#[tonic::async_trait]
impl CanIfaceService for NgCanIface {
    type IfaceStream = RxStream;

    async fn iface(
        &self,
        request: Request<Streaming<m::TxCanFrame>>,
    ) -> Result<Response<Self::IfaceStream>, Status> {
        todo!()
    }
}

pub fn get_ng_can_iface_server(shared: SharedHandle) -> CanIfaceServiceServer<NgCanIface> {
    CanIfaceServiceServer::new(NgCanIface { shared })
}
