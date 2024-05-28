use std::path::{Path, PathBuf};

use rocket::fs::{FileServer, NamedFile};
use rocket::serde::{Deserialize, Serialize};
use rocket::{log::LogLevel, Build, Config, Rocket};

use crate::shared::SharedHandle;

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
        log_level: LogLevel::Normal, // LogLevel::Critical
        cli_colors: true,
        port: config.port,
        address: config.listen.parse().unwrap(),
        ..Default::default()
    };

    let rocket = rocket::custom(config)
        .manage(shared)
        .mount("/", FileServer::from("ui/dist"));

    rocket
}
