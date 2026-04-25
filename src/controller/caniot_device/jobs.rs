use std::fmt::Debug;

use as_any::AsAny;
use chrono::{DateTime, Duration, Utc};
use cron::OwnedScheduleIterator;
use itertools::partition;

use crate::utils::{expirable::ExpirableTrait, Scheduling};

use super::DeviceMeasuresResetJob;

pub trait JobTrait: AsAny + Send + Sync + Debug {
    fn get_scheduling(&self) -> Scheduling {
        Scheduling::Unscheduled
    }
}

// Implement the trait for the unit type to avoid having a job for device
// controllers that don't need any
impl JobTrait for () {}

#[derive(Debug)]
pub enum DeviceJobImpl<'a, J>
where
    J: JobTrait,
{
    // Job executed when the device is added
    DeviceAdd,

    // Job executed when the device is removed
    DeviceRemoved,

    // Scheduled device specific job
    Scheduled(&'a J),
}

impl<'a, J> DeviceJobImpl<'a, J>
where
    J: JobTrait,
{
    pub fn is_device_add(&self) -> bool {
        matches!(self, DeviceJobImpl::DeviceAdd)
    }

    pub fn get_scheduled(&self) -> Option<&'a J> {
        match self {
            DeviceJobImpl::Scheduled(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum DeviceJobDefinition {
    DeviceAdd,
    DeviceRemove,
    Scheduled(Box<dyn JobTrait>),
}

impl DeviceJobDefinition {
    pub fn get_scheduling(&self) -> Scheduling {
        match self {
            DeviceJobDefinition::DeviceAdd => Scheduling::Immediate,
            DeviceJobDefinition::DeviceRemove => Scheduling::Immediate,
            DeviceJobDefinition::Scheduled(job) => job.get_scheduling(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
enum NextDatetime {
    Immediate,
    Never,
    At(DateTime<Utc>),
}

impl Ord for NextDatetime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (NextDatetime::Immediate, NextDatetime::Immediate) => std::cmp::Ordering::Equal,
            (NextDatetime::Immediate, _) => std::cmp::Ordering::Less,
            (_, NextDatetime::Immediate) => std::cmp::Ordering::Greater,
            (NextDatetime::Never, NextDatetime::Never) => std::cmp::Ordering::Equal,
            (NextDatetime::Never, _) => std::cmp::Ordering::Greater,
            (_, NextDatetime::Never) => std::cmp::Ordering::Less,
            (NextDatetime::At(dt1), NextDatetime::At(dt2)) => dt1.cmp(dt2),
        }
    }
}

impl NextDatetime {
    pub fn is_passed(&self, now: &DateTime<Utc>) -> bool {
        match self {
            NextDatetime::Immediate => true,
            NextDatetime::Never => false,
            NextDatetime::At(dt) => *dt <= *now,
        }
    }
}

impl ExpirableTrait<Duration> for NextDatetime {
    const ZERO: Duration = Duration::zero();
    type Instant = DateTime<Utc>;

    fn ttl(&self, now: &DateTime<Utc>) -> Option<Duration> {
        match self {
            NextDatetime::Immediate => Some(Self::ZERO),
            NextDatetime::Never => None,
            NextDatetime::At(dt) => {
                if *dt <= *now {
                    Some(Self::ZERO)
                } else {
                    Some(*dt - *now)
                }
            }
        }
    }
}

pub struct DeviceJobState {
    pub definition: DeviceJobDefinition,
    iterator: Option<OwnedScheduleIterator<Utc>>,
    next_occurrence: NextDatetime, // Datetime of the next occurence
}

impl DeviceJobState {
    pub fn init(start_dt: &DateTime<Utc>, definition: DeviceJobDefinition) -> Self {
        let (iterator, next_dt) = match definition.get_scheduling() {
            Scheduling::Immediate => (None, NextDatetime::Immediate),
            Scheduling::Unscheduled => (None, NextDatetime::Never),
            Scheduling::Cron(schedule) => {
                let mut iterator = schedule.after_owned(*start_dt);
                let next_dt = iterator
                    .next()
                    .map_or(NextDatetime::Never, NextDatetime::At);
                (Some(iterator), next_dt)
            }
        };

        Self {
            definition,
            iterator,
            next_occurrence: next_dt,
        }
    }

    pub fn reinit(&mut self, start_dt: &DateTime<Utc>) {
        let (iterator, next_dt) = match self.definition.get_scheduling() {
            Scheduling::Immediate => (None, NextDatetime::Immediate),
            Scheduling::Unscheduled => (None, NextDatetime::Never),
            Scheduling::Cron(schedule) => {
                let mut iterator = schedule.after_owned(*start_dt);
                let next_dt = iterator
                    .next()
                    .map_or(NextDatetime::Never, NextDatetime::At);
                (Some(iterator), next_dt)
            }
        };

        self.iterator = iterator;
        self.next_occurrence = next_dt;
    }

    pub fn is_ready(&self, now: &DateTime<Utc>) -> bool {
        self.next_occurrence.is_passed(now)
    }

    pub fn is_outdated(&self) -> bool {
        matches!(self.next_occurrence, NextDatetime::Never)
    }

    // Advance the job state to the next event
    pub fn advance(&mut self) {
        if let Some(iterator) = &mut self.iterator {
            let next_dt = iterator
                .next()
                .map_or(NextDatetime::Never, NextDatetime::At);
            self.next_occurrence = next_dt;
        } else {
            match self.definition.get_scheduling() {
                Scheduling::Immediate => self.next_occurrence = NextDatetime::Never,
                Scheduling::Unscheduled => self.next_occurrence = NextDatetime::Never,
                _ => {}
            }
        }
    }
}

impl ExpirableTrait<Duration> for DeviceJobState {
    const ZERO: Duration = Duration::zero();
    type Instant = DateTime<Utc>;

    fn ttl(&self, now: &DateTime<Utc>) -> Option<Duration> {
        let ttl = self.next_occurrence.ttl(now);
        ttl
    }
}

impl Debug for DeviceJobState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("def", &self.definition)
            .field("next ", &self.next_occurrence)
            .finish()
    }
}

pub struct DeviceJobsContext {
    scheduled_jobs: Vec<DeviceJobState>,

    // Keep track of the last time the jobs were evaluated (monitored)
    last_eval: DateTime<Utc>,
}

impl DeviceJobsContext {
    pub fn new(first_eval: DateTime<Utc>) -> Self {
        let init_jobs = vec![
            DeviceJobState::init(&first_eval, DeviceJobDefinition::DeviceAdd),
            DeviceJobState::init(
                &first_eval,
                DeviceJobDefinition::Scheduled(Box::new(DeviceMeasuresResetJob)),
            ),
        ];

        Self {
            last_eval: first_eval,
            scheduled_jobs: init_jobs,
        }
    }

    // Lots of processing happens here
    pub fn get_first_ready_job(&mut self, now: &DateTime<Utc>) -> Option<&mut DeviceJobState> {
        // Remove outdated jobs
        self.scheduled_jobs.retain_mut(|job| !job.is_outdated());

        // Partition the list of jobs in two parts: ready and unready jobs
        let partition_point = partition(&mut self.scheduled_jobs, |job| job.is_ready(now));

        // sort the ready jobs by their next occurrence (soonest first)
        self.scheduled_jobs[..partition_point].sort_by_key(|job| job.next_occurrence.clone());

        // Return the first ready job if it exists
        match partition_point {
            0 => None,
            _ => self.scheduled_jobs.get_mut(0),
        }
    }

    pub fn register_new_jobs(&mut self, jobs_definitions: Vec<Box<dyn JobTrait>>) {
        let new_definitions: Vec<DeviceJobState> = jobs_definitions
            .into_iter()
            .map(|job| DeviceJobDefinition::Scheduled(job))
            .map(|definition| DeviceJobState::init(&self.last_eval, definition))
            .collect();

        // Perform the registration + calculation if the jobs changed
        if !new_definitions.is_empty() {
            self.scheduled_jobs.extend(new_definitions);
        }
    }

    pub fn get_jobs_count(&self) -> usize {
        self.scheduled_jobs.len()
    }

    pub fn update_scheduled_jobs(
        &mut self,
        mut update_cb: impl FnMut(&mut DeviceJobDefinition) -> UpdateJobVerdict,
    ) {
        self.scheduled_jobs.retain_mut(|job| {
            let result = update_cb(&mut job.definition);

            match result {
                UpdateJobVerdict::Keep => {
                    job.reinit(&self.last_eval);
                    true
                }
                UpdateJobVerdict::Unschedule => false,
            }
        });
    }
}

impl ExpirableTrait<Duration> for DeviceJobsContext {
    const ZERO: Duration = Duration::zero();
    type Instant = DateTime<Utc>;

    fn ttl(&self, now: &DateTime<Utc>) -> Option<Duration> {
        self.scheduled_jobs
            .iter()
            .filter_map(|job| job.ttl(now))
            .min()
    }
}

#[derive(Debug)]
pub enum UpdateJobVerdict {
    Keep,
    Unschedule,
}

pub fn downcast_job_as<J: JobTrait>(job: &Box<dyn JobTrait>) -> Option<&J> {
    job.as_ref().as_any().downcast_ref::<J>()
}
