use chrono::{DateTime, Utc};

pub enum FirmwareStatus {
    Running,
    Updating,
    Error,
}

pub struct FirmwareBuildInfos {
    pub version: String,
    pub commit_hash: String,
    pub build_date: DateTime<Utc>,
    pub status: FirmwareStatus,
}

pub struct FirmwareInfos {
    pub build: FirmwareBuildInfos,
    pub status: FirmwareStatus,
    pub start_date: DateTime<Utc>,

    // Last date the firmware has been updated
    pub update_date: Option<DateTime<Utc>>,
}
