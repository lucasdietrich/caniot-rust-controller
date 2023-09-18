use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{log::LogLevel, Build, Config, Rocket};

use crate::config::ServerConfig;
use crate::context::ContextHandle;

use crate::server::rest::*;

pub fn rocket(config: ServerConfig, context: ContextHandle) -> Rocket<Build> {
    let config = Config {
        workers: 1,
        log_level: LogLevel::Critical,
        port: config.port,
        address: config.listen.parse().unwrap(),
        cli_colors: false,
        ..Default::default()
    };

    rocket::custom(config)
        .manage(context)
        .mount("/", routes![route_test, route_test_id_name, route_stats])
}
