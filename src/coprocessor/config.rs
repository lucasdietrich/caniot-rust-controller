use serde::{Deserialize, Serialize};

pub use ble_copro_stream_server::{DEFAULT_LISTEN_IP, DEFAULT_LISTEN_PORT};

fn default_listen_ip() -> String {
    DEFAULT_LISTEN_IP.to_string()
}

const fn default_listen_port() -> u16 {
    DEFAULT_LISTEN_PORT
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CoproConfig {
    #[serde(default = "default_listen_ip")]
    pub listen_ip: String,
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,
}
