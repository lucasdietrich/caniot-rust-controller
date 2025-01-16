use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

use crate::bus::CanConfig;
use crate::controller::CaniotConfig;
use crate::coprocessor::CoproConfig;
use crate::database::DatabaseConfig;
use crate::ha::ha::HaConfig;
use crate::webserver::WebserverConfig;

use crate::grpcserver::GrpcConfig;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppConfig {
    pub can: CanConfig,
    pub caniot: CaniotConfig,
    pub copro: CoproConfig,
    pub web: WebserverConfig,
    pub database: DatabaseConfig,
    pub grpc: GrpcConfig,
    pub ha: HaConfig,
}

const CONFIG_PATH: &str = "caniot-controller.toml";

/// The function `load_config` reads a configuration file, parses its contents as TOML, and returns a
/// `AppConfig` struct.
///
/// Returns:
///
/// The function `load_config()` is returning a value of type `AppConfig`.
pub fn load_config(config: Option<&str>) -> AppConfig {
    let config_file = config.unwrap_or(CONFIG_PATH);
    match fs::read_to_string(config_file) {
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
