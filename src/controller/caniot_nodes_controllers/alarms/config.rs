use chrono::NaiveTime;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    controller::{ConfigTrait, DeviceError, PartialConfigTrait},
    database::{SettingsError, SettingsStore},
};

use super::jobs::{AlarmJob, AutoAction, AutoDevice};

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct AlarmDetectionTimeRangeConfig {
    lower_bound: NaiveTime,
    upper_bound: NaiveTime,
    week_days: Vec<u32>,
    detections_to_trigger_east: u32,
    detections_to_trigger_south: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    pub auto_alarm_enable: bool,
    pub auto_alarm_enable_time: NaiveTime,
    pub auto_alarm_disable_time: NaiveTime,
    pub alarm_siren_minimum_interval_seconds: u32,

    pub auto_lights_enable: bool,
    pub auto_lights_enable_time: NaiveTime,
    pub auto_lights_disable_time: NaiveTime,
    // pub detection_time_ranges: Vec<AlarmDetectionTimeRangeConfig>,
}

impl Default for AlarmConfig {
    fn default() -> Self {
        Self {
            auto_alarm_enable: true,
            auto_alarm_enable_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
            auto_alarm_disable_time: NaiveTime::from_hms_opt(6, 0, 0).unwrap_or_default(),
            alarm_siren_minimum_interval_seconds: 30,

            auto_lights_enable: true,
            auto_lights_enable_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap_or_default(),
            auto_lights_disable_time: NaiveTime::from_hms_opt(6, 0, 0).unwrap_or_default(),
            // detection_time_ranges: vec![],
        }
    }
}

// Patch value with new value if it is different and return the new value
// or do nothing if the value is the same and return None
fn patch_val<T: Eq + Clone>(val: &mut T, new_val: Option<T>) -> Option<T> {
    if let Some(new_val) = new_val {
        if val != &new_val {
            *val = new_val.clone();
            Some(new_val)
        } else {
            None
        }
    } else {
        None
    }
}

#[async_trait]
impl ConfigTrait for AlarmConfig {
    type PartialConfig = AlarmPartialConfig;
    type PatchVerdict = AlarmUpdateResult;

    fn patch(&mut self, partial: Self::PartialConfig) -> Result<AlarmUpdateResult, DeviceError> {
        fn process_auto_device(
            enable_time: &mut NaiveTime,
            partial_enable_time: Option<NaiveTime>,
            disable_time: &mut NaiveTime,
            partial_disable_time: Option<NaiveTime>,
            enable: &mut bool,
            partial_enable: Option<bool>,
            device: AutoDevice,
        ) -> ConfigUpdateAutoJobsResult {
            let time_changed = patch_val(enable_time, partial_enable_time).is_some()
                || patch_val(disable_time, partial_disable_time).is_some();

            if let Some(v) = patch_val(enable, partial_enable) {
                match v {
                    true => ConfigUpdateAutoJobsResult::EnableJobs(
                        AlarmJob::DailyAuto(*enable_time, device, AutoAction::Enable),
                        AlarmJob::DailyAuto(*disable_time, device, AutoAction::Disable),
                    ),
                    false => ConfigUpdateAutoJobsResult::UpdateJobs,
                }
            } else if time_changed {
                ConfigUpdateAutoJobsResult::UpdateJobs
            } else {
                ConfigUpdateAutoJobsResult::Unchanged
            }
        }

        let auto_alarm_result = process_auto_device(
            &mut self.auto_alarm_enable_time,
            partial.auto_alarm_enable_time,
            &mut self.auto_alarm_disable_time,
            partial.auto_alarm_disable_time,
            &mut self.auto_alarm_enable,
            partial.auto_alarm_enable,
            AutoDevice::Alarm,
        );

        let auto_lights_result = process_auto_device(
            &mut self.auto_lights_enable_time,
            partial.auto_lights_enable_time,
            &mut self.auto_lights_disable_time,
            partial.auto_lights_disable_time,
            &mut self.auto_lights_enable,
            partial.auto_lights_enable,
            AutoDevice::Lights,
        );

        if let Some(alarm_siren_minimum_interval_seconds) =
            partial.alarm_siren_minimum_interval_seconds
        {
            self.alarm_siren_minimum_interval_seconds = alarm_siren_minimum_interval_seconds;
        }
        // if let Some(detection_time_ranges) = partial.detection_time_ranges.as_ref() {
        //     self.detection_time_ranges = detection_time_ranges.clone();
        // }

        Ok(AlarmUpdateResult {
            alarm: auto_alarm_result,
            lights: auto_lights_result,
        })
    }

    async fn load<'a>(stg: &SettingsStore<'a>) -> Result<Self, SettingsError> {
        let default = AlarmConfig::default();

        let auto_alarm_enable = stg
            .read_or("auto_alarm_enable", default.auto_alarm_enable)
            .await?;
        let auto_alarm_enable_time = stg
            .read_or("auto_alarm_enable_time", default.auto_alarm_enable_time)
            .await?;
        let auto_alarm_disable_time = stg
            .read_or("auto_alarm_disable_time", default.auto_alarm_disable_time)
            .await?;
        let alarm_siren_minimum_interval_seconds = stg
            .read_or(
                "alarm_siren_minimum_interval_seconds",
                default.alarm_siren_minimum_interval_seconds,
            )
            .await?;

        let auto_lights_enable = stg
            .read_or("auto_lights_enable", default.auto_lights_enable)
            .await?;
        let auto_lights_enable_time = stg
            .read_or("auto_lights_enable_time", default.auto_lights_enable_time)
            .await?;
        let auto_lights_disable_time = stg
            .read_or("auto_lights_disable_time", default.auto_lights_disable_time)
            .await?;

        Ok(Self {
            auto_alarm_enable,
            auto_alarm_enable_time,
            auto_alarm_disable_time,
            alarm_siren_minimum_interval_seconds,
            auto_lights_enable,
            auto_lights_enable_time,
            auto_lights_disable_time,
        })
    }

    fn into_patch(&self) -> Self::PartialConfig {
        AlarmPartialConfig {
            auto_alarm_enable: Some(self.auto_alarm_enable),
            auto_alarm_enable_time: Some(self.auto_alarm_enable_time),
            auto_alarm_disable_time: Some(self.auto_alarm_disable_time),
            alarm_siren_minimum_interval_seconds: Some(self.alarm_siren_minimum_interval_seconds),
            auto_lights_enable: Some(self.auto_lights_enable),
            auto_lights_enable_time: Some(self.auto_lights_enable_time),
            auto_lights_disable_time: Some(self.auto_lights_disable_time),
            // detection_time_ranges: Some(self.detection_time_ranges.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct AlarmPartialConfig {
    pub auto_alarm_enable: Option<bool>,
    pub auto_alarm_enable_time: Option<NaiveTime>,
    pub auto_alarm_disable_time: Option<NaiveTime>,
    pub alarm_siren_minimum_interval_seconds: Option<u32>,

    pub auto_lights_enable: Option<bool>,
    pub auto_lights_enable_time: Option<NaiveTime>,
    pub auto_lights_disable_time: Option<NaiveTime>,
    // pub detection_time_ranges: Option<Vec<AlarmDetectionTimeRangeConfig>>,
}

#[async_trait]
impl PartialConfigTrait for AlarmPartialConfig {
    async fn save<'a>(&self, stg: &SettingsStore<'a>) -> Result<(), SettingsError> {
        debug!("Saving alarm partial config: {:#?}", self);

        if let Some(auto_alarm_enable) = self.auto_alarm_enable {
            stg.write("auto_alarm_enable", &auto_alarm_enable).await?;
        }

        if let Some(auto_alarm_enable_time) = self.auto_alarm_enable_time {
            stg.write("auto_alarm_enable_time", &auto_alarm_enable_time)
                .await?;
        }

        if let Some(auto_alarm_disable_time) = self.auto_alarm_disable_time {
            stg.write("auto_alarm_disable_time", &auto_alarm_disable_time)
                .await?;
        }

        if let Some(alarm_siren_minimum_interval_seconds) =
            self.alarm_siren_minimum_interval_seconds
        {
            stg.write(
                "alarm_siren_minimum_interval_seconds",
                &alarm_siren_minimum_interval_seconds,
            )
            .await?;
        }

        if let Some(auto_lights_enable) = self.auto_lights_enable {
            stg.write("auto_lights_enable", &auto_lights_enable).await?;
        }

        if let Some(auto_lights_enable_time) = self.auto_lights_enable_time {
            stg.write("auto_lights_enable_time", &auto_lights_enable_time)
                .await?;
        }

        if let Some(auto_lights_disable_time) = self.auto_lights_disable_time {
            stg.write("auto_lights_disable_time", &auto_lights_disable_time)
                .await?;
        }

        Ok(())
    }
}

pub enum ConfigUpdateAutoJobsResult {
    Unchanged,
    EnableJobs(AlarmJob, AlarmJob),
    UpdateJobs,
}

pub struct AlarmUpdateResult {
    pub alarm: ConfigUpdateAutoJobsResult,
    pub lights: ConfigUpdateAutoJobsResult,
}
