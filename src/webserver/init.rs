use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
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

// Copyright kirauks
#[get("/<path..>")]
pub async fn files(path: PathBuf) -> Option<NamedFile> {
    let www = "ui/dist";

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
        .mount("/", routes![files]);

    rocket
}
