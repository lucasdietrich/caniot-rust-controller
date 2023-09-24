use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{log::LogLevel, Build, Config, Rocket};

use crate::shared::SharedHandle;
use super::rest::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebserverConfig {
    pub port: u16,
    pub listen: String,
}

impl Default for WebserverConfig {
    fn default() -> Self {
        WebserverConfig {
            port: 8000,
            listen: "0.0.0.0".to_string(),
        }
    }
}

pub fn rocket(shared: SharedHandle) -> Rocket<Build> {
    let config = &shared.config.web;
    let config = Config {
        workers: 1,
        log_level: LogLevel::Critical,
        port: config.port,
        address: config.listen.parse().unwrap(),
        cli_colors: false,
        ..Default::default()
    };

    rocket::custom(config).manage(shared).mount(
        "/",
        routes![route_test, route_test_id_name, route_stats, route_can, route_config],
    )
}
