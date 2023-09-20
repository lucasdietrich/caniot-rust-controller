use crate::shared::{SharedHandle, Stats};
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    State,
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
