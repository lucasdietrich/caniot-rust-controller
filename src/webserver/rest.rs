use rocket::{
    response::content::{self, RawJson},
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use crate::{caniot::Request as CaniotRequest, shared::ServerStats};
use crate::{
    caniot::{DeviceId, Endpoint},
    shared::{SharedHandle, Stats},
};

use num_derive::FromPrimitive;

#[get("/test")]
pub fn route_test() -> &'static str {
    "Hello world!"
}

// match regexp "/test/([0-9]+)/name/([a-z]+)"
#[get("/test/<id>/name/<name>")]
pub fn route_test_id_name(id: u32, name: &str) -> String {
    format!("Hello {}! Your id is {}", name, id)
}

#[get("/stats")]
pub async fn route_stats(shared: &State<SharedHandle>) -> Json<Stats> {
    let (caniot, can) = shared.controller_handle.get_stats().await.unwrap();

    let stats = Stats {
        caniot,
        can,
        server: ServerStats {},
    };

    Json(stats)
}

#[get("/caniot/request_telemetry/<did>/<endpoint>")]
pub async fn route_caniot_request_telemetry(
    did: u8,
    endpoint: u8,
    shared: &State<SharedHandle>,
) -> content::RawJson<&'static str> {
    let device_id = DeviceId::from(did);
    let endpoint = num::FromPrimitive::from_u8(endpoint).unwrap();

    let response = shared
        .controller_handle
        .query_telemetry(device_id, endpoint, 1000)
        .await
        .unwrap();

    if let Some(response) = response {
        println!("response: {:?}", response);
        RawJson("{ \"status\": \"ok\" }")
    } else {
        RawJson("{ \"status\": \"timeout\" }")
    }
}

#[get("/config")]
pub fn route_config(shared: &State<SharedHandle>) -> Json<crate::config::AppConfig> {
    let config = &shared.config;
    Json(config.clone())
}

// #[derive(Serialize, Deserialize, Debug)]
// struct CanFrame {
//     device_id: u8,
// }

#[post("/can/test/<did>")]
pub async fn route_can(did: u8, shared: &State<SharedHandle>) -> Result<(), String> {
    let caniot_request = CaniotRequest {
        device_id: DeviceId::from(did),
        data: crate::caniot::RequestData::Telemetry {
            endpoint: crate::caniot::Endpoint::BoardControl,
        },
    };

    let z = shared.controller_handle.query().await;

    Ok(())
}
