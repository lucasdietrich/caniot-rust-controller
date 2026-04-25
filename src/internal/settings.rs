use serde::Serialize;

// global settings
#[allow(dead_code)]
#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct Settings {
    pub dark_mode: bool,
    pub debug_mode: bool,
}
