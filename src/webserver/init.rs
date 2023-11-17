use std::path::Path;

use rocket::fs::FileServer;
use rocket::serde::{Deserialize, Serialize};
use rocket::{log::LogLevel, Build, Config, Rocket};
use rocket_dyn_templates::Template;

use super::rest::*;
use super::web;
use crate::shared::SharedHandle;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebserverConfig {
    pub port: u16,
    pub listen: String,
    pub static_path: String,
    // pub templates_path: String,
}

impl Default for WebserverConfig {
    fn default() -> Self {
        WebserverConfig {
            port: 8000,
            listen: "0.0.0.0".to_string(),
            static_path: "static".to_string(),
            // templates_path: "templates".to_string(),
        }
    }
}

pub fn rocket(shared: SharedHandle) -> Rocket<Build> {
    let config = &shared.config.web;

    let static_path = config.static_path.clone();
    let static_path = Path::new(&static_path);
    let static_routes = if static_path.exists() {
        FileServer::from(static_path).into()
    } else {
        vec![]
    };

    let config = Config {
        workers: 1,
        log_level: LogLevel::Off, // LogLevel::Critical
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
        .mount("/", static_routes)
        .mount("/", routes![web::web_hello,])
        .attach(Template::fairing())
}
