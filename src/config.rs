use log::warn;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug, Default)]
pub struct AppConfig {
    pub can: CanConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct CanConfig {
    pub interface: String,
}

impl Default for CanConfig {
    fn default() -> Self {
        CanConfig {
            interface: "vcan0".to_string(),
        }
    }
}

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
