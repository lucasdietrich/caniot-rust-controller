use std::path::{Path, PathBuf};

use log::warn;
use rocket::config::Shutdown;
use rocket::fs::NamedFile;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{log::LogLevel, Config, Rocket};

use crate::shared::SharedHandle;

use super::prometheus_exporter;

const DEFAULT_PORT: u16 = 8000;
const DEFAULT_LISTEN: &str = "0.0.0.0";
const DEFAULT_STATIC_PATH: &str = "ui/dist";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebserverConfig {
    pub port: u16,
    pub listen: String,
    pub static_path: String,

    // Enable prometheus metrics
    pub prometheus_metrics: bool,
}

impl Default for WebserverConfig {
    fn default() -> Self {
        WebserverConfig {
            port: DEFAULT_PORT,
            listen: DEFAULT_LISTEN.to_string(),
            static_path: DEFAULT_STATIC_PATH.to_string(),
            prometheus_metrics: true,
        }
    }
}

// Copyright kirauks
#[get("/<path..>")]
pub async fn files(path: PathBuf, state: &State<SharedHandle>) -> Option<NamedFile> {
    let www: &str = &state.config.web.static_path.as_ref();

    let mut path = Path::new(www).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    match NamedFile::open(path).await {
        Ok(named_file) => Some(named_file),
        Err(_) => NamedFile::open(Path::new(www).join("index.html"))
            .await
            .ok(),
    }
}

// prometheus exporter
#[get("/metrics")]
pub async fn metrics(shared: &State<SharedHandle>) -> String {
    if shared.config.web.prometheus_metrics {
        prometheus_exporter::export(shared).await
    } else {
        warn!("Accessing /metrics but prometheus exporter is disabled");
        "".to_owned()
    }
}

pub async fn rocket_server(shared: SharedHandle) -> Result<Rocket<rocket::Ignite>, rocket::Error> {
    let config = &shared.config.web;

    let config = Config {
        workers: 1,
        log_level: LogLevel::Normal, // LogLevel::Critical
        cli_colors: true,
        port: config.port,
        address: config.listen.parse().expect("Invalid address"),
        shutdown: Shutdown {
            ctrlc: true,
            grace: 2,
            mercy: 3,
            force: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let rocket = rocket::custom(config)
        .manage(shared)
        .mount("/", routes![metrics, files]);

    rocket.launch().await
}
