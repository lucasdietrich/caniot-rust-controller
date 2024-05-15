use chrono::{DateTime, Utc};

pub enum FirmwareStatus {
    Running,
    Updating,
    Error,
}

pub struct FirmwareInfos {
    pub version: String,
    pub build_date: DateTime<Utc>,
    pub status: FirmwareStatus,
}
