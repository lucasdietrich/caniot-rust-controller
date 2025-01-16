use std::u32;

use serde::{Deserialize, Serialize};

pub const fn device_default_order() -> u32 {
    u32::MAX
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CoproDeviceConfig {
    pub name: String,
    pub mac: String,
    #[serde(default = "device_default_order")]
    pub ui_display_order: u32,
    pub location: Option<String>,
}
