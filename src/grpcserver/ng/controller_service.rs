use std::ops::BitAnd;

use tonic::{Request, Response, Result, Status};

use crate::caniot::{self};
use crate::controller::ControllerError;
use crate::shared::SharedHandle;

use super::model::controller::{
    self as m,
    controller_service_server::{ControllerService, ControllerServiceServer},
};

#[derive(Debug)]
pub struct NgController {
    pub shared: SharedHandle,
}

fn convert_payload<'a, T>(payload: &'a [T]) -> Vec<u8>
where
    &'a T: BitAnd<u32, Output = u32> + Copy,
{
    let mut bytes = Vec::new();
    for p in &payload[..8] {
        bytes.push((p & 0xff) as u8);
    }
    bytes
}

#[tonic::async_trait]
impl ControllerService for NgController {
    async fn query(&self, request: Request<m::Request>) -> Result<Response<m::Response>, Status> {
        let req = request.into_inner();
        let did = req.did.expect("Missing device id");
        let caniot_did = caniot::DeviceId::from_u8(did.did as u8);

        let query = match req.query.expect("Missing query") {
            m::request::Query::Telemetry(t) => {
                let ep = caniot::Endpoint::try_from(t.endpoint).expect("Invalid endpoint");
                caniot::build_telemetry_request(caniot_did, ep)
            }
            m::request::Query::Command(a) => {
                let ep = caniot::Endpoint::try_from(a.endpoint).expect("Invalid endpoint");
                caniot::build_command_request(caniot_did, ep, convert_payload(a.payload.as_slice()))
            }
            m::request::Query::Attribute(a) => match a.value {
                Some(value) => {
                    caniot::build_attribute_write_request(caniot_did, a.key as u16, value)
                }
                None => caniot::build_attribute_read_request(caniot_did, a.key as u16),
            },
        };

        let reply = self
            .shared
            .controller_handle
            .device_request(query, req.timeout)
            .await;

        // Handle request error
        if let Err(err) = reply {
            return Ok(Response::new(m::Response {
                did: Some(caniot_did.into()),
                response: None,
                response_time: 0,
                status: match err {
                    ControllerError::Timeout => m::Status::Timeout as i32,
                    _ => m::Status::Nok as i32,
                },
                timestamp: None,
            }));
        }

        // Handle device response
        let _response = reply.unwrap();

        // let resp = match result.unwrap().data {
        //     ResponseData::Telemetry { endpoint, payload } => {}
        //     ResponseData::Attribute { key, value } => {}
        //     ResponseData::Error { source, error } => {}
        // };

        // let r1 = model::response::Response::Telemetry(model::Telemetry {
        //     endpoint: 0,
        //     payload: Vec::new(),
        // });

        // let r2 = model::response::Response::Attribute(model::Attribute { key: 0, value: 0 });

        Ok(Response::new(m::Response {
            did: Some(caniot_did.into()),
            response: None,
            response_time: 0,
            status: m::Status::Ok as i32,
            timestamp: None,
        }))
    }
}

pub fn get_ng_controller_server(shared: SharedHandle) -> ControllerServiceServer<NgController> {
    ControllerServiceServer::new(NgController { shared })
}
