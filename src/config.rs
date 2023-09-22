use log::warn;
use serde::Deserialize;
use std::fs;
use toml;

use crate::can::CanConfig;
use crate::server::ServerConfig;

#[derive(Deserialize, Debug, Default)]
pub struct AppConfig {
    pub can: CanConfig,
    pub server: ServerConfig,
}

const CONFIG_PATH: &str = "caniot-controller.toml";

/// The function `load_config` reads a configuration file, parses its contents as TOML, and returns a
/// `AppConfig` struct.
///
/// Returns:
///
/// The function `load_config()` is returning a value of type `AppConfig`.
pub fn load_config() -> AppConfig {
    match fs::read_to_string(CONFIG_PATH) {
        Ok(content) => toml::from_str::<AppConfig>(&content).unwrap_or_else(|e| {
            warn!("Could not parse config file: {}", e);
            AppConfig::default()
        }),
        Err(e) => {
            warn!("Could not read config file {}, err:{}", CONFIG_PATH, e);
            AppConfig::default()
        }
    }
}
