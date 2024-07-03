use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CaniotDevicesConfig {
    pub demo_did: Option<u8>,
    pub heaters_did: Option<u8>,
    pub garage_did: Option<u8>,
    pub outdoor_alarm_did: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CaniotConfig {
    pub pending_queries_default_timeout: Option<u32>, // s
    pub action_default_timeout: Option<u32>,          // s
    pub inernal_api_mpsc_size: Option<u32>,

    pub devices: CaniotDevicesConfig,
}
