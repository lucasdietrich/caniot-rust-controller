use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use crate::caniot::Request as CaniotRequest;
use crate::{
    caniot::DeviceId,
    shared::{SharedHandle, Stats},
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

#[get("/stats")]
pub fn route_stats(shared: &State<SharedHandle>) -> Json<Stats> {
    let stats = shared.stats.lock().unwrap();
    Json(stats.clone())
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

    shared
        .can_tx_queue
        .clone()
        .send(caniot_request)
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}
