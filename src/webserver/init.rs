use std::ops::Deref;
use std::path::Path;

use rocket::fs::FileServer;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{log::LogLevel, Build, Config, Rocket};

use super::rest::*;
use crate::shared::SharedHandle;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebserverConfig {
    pub port: u16,
    pub listen: String,
    pub static_path: String,
}

impl Default for WebserverConfig {
    fn default() -> Self {
        WebserverConfig {
            port: 8000,
            listen: "0.0.0.0".to_string(),
            static_path: "static".to_string(),
        }
    }
}

pub fn rocket(shared: SharedHandle) -> Rocket<Build> {
    let config = &shared.config.web;
    let static_path = config.static_path.clone();
    let static_path = Path::new(&static_path);

    let config = Config {
        workers: 1,
        log_level: LogLevel::Normal, // LogLevel::Critical
        port: config.port,
        address: config.listen.parse().unwrap(),
        cli_colors: false,
        ..Default::default()
    };

    rocket::custom(config)
        .manage(shared)
        .mount(
            "/api/",
            routes![
                route_test,
                route_test_id_name,
                route_stats,
                route_can,
                route_config,
                route_caniot_request_telemetry,
            ],
        )
        .mount("/", FileServer::from(static_path))
}
