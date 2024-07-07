use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct SoftwareBuildInfos {
    pub version: Option<String>,
    pub commit_hash: Option<String>,
    pub build_date: Option<DateTime<Utc>>,

    // tell whether the git workspace is dirty
    pub git_dirty: bool,
}

impl SoftwareBuildInfos {
    pub fn is_complete(&self) -> bool {
        self.version.is_some() && self.commit_hash.is_some() && self.build_date.is_some()
    }

    pub fn is_dirty(&self) -> bool {
        self.git_dirty
    }

    pub fn get_commit_hash_and_dirty(&self) -> Option<String> {
        self.commit_hash.as_ref().map(|s| {
            if self.git_dirty {
                format!("{} (dirty)", s)
            } else {
                s.to_owned()
            }
        })
    }
}

impl Default for SoftwareBuildInfos {
    fn default() -> Self {
        let build_commit = option_env!("CANIOT_CONTROLLER_GIT_HASH");
        let build_date = option_env!("CANIOT_CONTROLLER_BUILD_DATE");
        let cargo_version = option_env!("CARGO_PKG_VERSION");
        let git_dirty = option_env!("CANIOT_CONTROLLER_GIT_DIRTY").unwrap_or("false") == "true";

        // parse as seconds since epoch (UTC) : e.g. 1719567302
        let build_date = build_date.map(|s| {
            let naive = NaiveDateTime::from_timestamp_opt(s.parse::<i64>().unwrap(), 0)
                .expect("Failed to parse build date");
            DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
        });

        Self {
            commit_hash: build_commit.map(|s| s.to_string()),
            build_date,
            version: cargo_version.map(|s| s.to_string()),
            git_dirty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoftwareRuntimeInfos {
    pub start_time: DateTime<Utc>,
}

impl Default for SoftwareRuntimeInfos {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            start_time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoftwareInfos {
    pub build: SoftwareBuildInfos,

    // Last date the software has been updated
    pub update_date: Option<DateTime<Utc>>,

    // Runtime infos
    pub runtime: SoftwareRuntimeInfos,
}

impl Default for SoftwareInfos {
    fn default() -> Self {
        Self {
            build: SoftwareBuildInfos::default(),
            update_date: None,
            runtime: SoftwareRuntimeInfos::default(),
        }
    }
}
