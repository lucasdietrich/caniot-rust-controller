use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CoproDeviceConfig {
    pub name: String,
    pub mac: String,
}
