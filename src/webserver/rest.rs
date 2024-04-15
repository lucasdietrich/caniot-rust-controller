

use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    Responder, State,
};

use crate::controller::{self, DeviceStatsEntry};
use crate::{
    bus::CanStats,
    caniot,
    shared::{self, ServerStats},
};
use crate::{
    caniot::{DeviceId},
    shared::SharedHandle,
};

#[get("/test")]
pub fn route_test() -> &'static str {
    "Hello world!"
}

// match regexp "/test/([0-9]+)/name/([a-z]+)"
#[get("/test/<id>/name/<name>")]
pub fn route_test_id_name(id: u32, name: &str) -> String {
    format!("Hello {}! Your id is {}", name, id)
}

#[derive(Serialize, Debug, Clone)]
pub struct Stats {
    pub controller: controller::ControllerStats,
    pub devices: Vec<DeviceStatsEntry>,
    pub can: CanStats,
    pub server: ServerStats,
}

#[get("/stats")]
pub async fn route_stats(shared: &State<SharedHandle>) -> Json<Stats> {
    let (controller, devices, can) = shared.controller_handle.get_stats().await;

    let stats = Stats {
        controller,
        devices,
        can,
        server: shared::ServerStats {},
    };

    Json(stats)
}

#[derive(Serialize, Deserialize, Debug)]
struct CaniotResp {
    status: String,
}

#[derive(Responder)]
pub enum CaniotRestResponse {
    #[response(status = 200, content_type = "json")]
    Ok(Json<caniot::Response>),

    #[response(status = 404)]
    Timeout(String),

    #[response(status = 400)]
    Error(String),
}

#[get("/caniot/request_telemetry/<did>/<endpoint>")]
pub async fn route_caniot_request_telemetry(
    did: u8,
    endpoint: u8,
    shared: &State<SharedHandle>,
) -> CaniotRestResponse {
    let _did = match DeviceId::try_from(did) {
        Ok(did) => did,
        Err(_) => return CaniotRestResponse::Error("Invalid device id".to_string()), // willingly ignoring protocol error
    };

    let _endpoint = endpoint;
    let _shared = shared;

    // let endpoint = if let Some(endpoint) = num::FromPrimitive::from_u8(endpoint) {
    //     endpoint
    // } else {
    //     return CaniotRestResponse::Error("Invalid endpoint".to_string());
    // };

    // match shared
    //     .controller_handle
    //     .get_device(did)
    //     .request_telemetry(endpoint, 1000)
    //     .await
    // {
    //     Ok(response) => CaniotRestResponse::Ok(Json(response)),
    //     Err(ControllerError::Timeout) => CaniotRestResponse::Timeout("Error: Timeout".to_string()),
    //     Err(e) => CaniotRestResponse::Error(format!("Error: {}", e)),
    // }

    CaniotRestResponse::Error(format!("Error"))
}

// #[get("/caniot/request_telemetry/<did>/<endpoint>")]
// pub async fn route_caniot_request_telemetry(
//     did: u8,
//     endpoint: u8,
//     shared: &State<SharedHandle>,
// ) -> content::RawJson<&'static str> {
//     let device_id = DeviceId::try_from(did).unwrap();
//     let endpoint = num::FromPrimitive::from_u8(endpoint).unwrap();

//     let response = shared
//         .controller_handle
//         .query_telemetry(device_id, endpoint, 1000)
//         .await;

//     if let Ok(response) = response {
//         info!("response: {:?}", response);
//         RawJson("{ \"status\": \"ok\" }")
//     } else {
//         RawJson("{ \"status\": \"timeout\" }")
//     }
// }

#[get("/config")]
pub fn route_config(shared: &State<SharedHandle>) -> Json<crate::config::AppConfig> {
    let config = &shared.config;
    Json(config.clone())
}

// #[derive(Serialize, Deserialize, Debug)]
// struct CanFrame {
//     device_id: u8,
// }
