use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{log::LogLevel, Build, Config, Rocket};

use crate::shared::SharedHandle;
use crate::webserver::rest::*;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub listen: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            port: 8000,
            listen: "0.0.0.0".to_string(),
        }
    }
}

pub fn rocket(config: ServerConfig, shared: SharedHandle) -> Rocket<Build> {
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
        routes![route_test, route_test_id_name, route_stats, route_can],
    )
}
