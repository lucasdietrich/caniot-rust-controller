use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use cron::Schedule;

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub enum Scheduling {
    #[default]
    Unscheduled,
    Immediate,
    Cron(Schedule), // Scheduling should return OwnedScheduleIterator instead of Schedule
}

impl Scheduling {
    pub fn new_cron(schedule: &str) -> Self {
        Scheduling::Cron(Schedule::from_str(schedule).expect("Invalid cron schedule"))
    }

    pub fn into_next(self) -> Self {
        match self {
            Scheduling::Unscheduled | Scheduling::Immediate => Scheduling::Unscheduled,
            _ => self,
        }
    }

    pub fn is_unscheduled(&self) -> bool {
        match self {
            Scheduling::Unscheduled => true,
            _ => false,
        }
    }
}
