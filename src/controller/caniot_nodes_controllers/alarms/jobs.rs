use chrono::NaiveTime;

use crate::{controller::JobTrait, utils::Scheduling};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoDevice {
    Alarm,
    Lights,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoAction {
    Enable,
    Disable,
}

#[derive(Debug, Clone)]
pub enum AlarmJob {
    DailyAuto(NaiveTime, AutoDevice, AutoAction),
}

impl JobTrait for AlarmJob {
    fn get_scheduling(&self) -> Scheduling {
        match self {
            AlarmJob::DailyAuto(time, ..) => Scheduling::Daily(*time),
        }
    }
}
