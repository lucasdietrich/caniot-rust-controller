use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum FirmwareStatus {
    Running,
    Updating,
    Error,
}

#[derive(Debug, Clone)]
pub struct FirmwareBuildInfos {
    pub distro: String,
    pub distro_version: String,
    pub build_date: Option<DateTime<Utc>>,
}

impl Default for FirmwareBuildInfos {
    fn default() -> Self {
        Self {
            distro: "hypirl".to_string(),
            distro_version: "scarthgap".to_string(),
            build_date: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirmwareInfos {
    pub build: FirmwareBuildInfos,
}

impl Default for FirmwareInfos {
    fn default() -> Self {
        Self {
            build: FirmwareBuildInfos::default(),
        }
    }
}
