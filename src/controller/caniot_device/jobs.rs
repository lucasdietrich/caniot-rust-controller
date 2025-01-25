use std::{fmt::Debug, thread::sleep};

use as_any::AsAny;
use chrono::{DateTime, Duration, Utc};
use cron::OwnedScheduleIterator;
use itertools::{partition, Itertools};
use log::debug;

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

#[derive(Debug)]
enum NextDatetime {
    Immediate,
    Never,
    At(DateTime<Utc>),
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

pub struct JobsIterator<'a> {
    ready_jobs: std::slice::IterMut<'a, DeviceJobState>,
}

impl<'a> JobsIterator<'a> {
    pub fn new(ready_jobs: &'a mut [DeviceJobState]) -> Self {
        Self {
            ready_jobs: ready_jobs.iter_mut(),
        }
    }
}

impl<'a> Iterator for JobsIterator<'a> {
    type Item = &'a mut DeviceJobState;

    fn next(&mut self) -> Option<Self::Item> {
        self.ready_jobs.next()
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

    pub fn advance(&mut self) {
        if let Some(iterator) = &mut self.iterator {
            let next_dt = iterator
                .next()
                .map_or(NextDatetime::Never, NextDatetime::At);
            println!("Advancing job {:?} to {:?}", self.definition, next_dt);
            self.next_occurrence = next_dt;
        } else {
            match self.definition.get_scheduling() {
                Scheduling::Immediate => self.next_occurrence = NextDatetime::Never,
                Scheduling::Unscheduled => self.next_occurrence = NextDatetime::Never,
                _ => {}
            }
        }

        sleep(std::time::Duration::from_millis(500));
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

    // Optimization to avoid re-evaluating the jobs too often
    eval_in: Option<Duration>,
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

        let init_eval_in = init_jobs.ttl(&first_eval);

        Self {
            last_eval: first_eval,
            eval_in: init_eval_in,
            scheduled_jobs: init_jobs,
        }
    }

    // Remove outdated jobs and return an iterator over the ready jobs
    pub fn monitor_ready_jobs(&mut self, now: &DateTime<Utc>) -> JobsIterator {
        // Remove outdated jobs
        self.scheduled_jobs.retain_mut(|job| !job.is_outdated());

        // Partition the list of jobs in two parts: ready and unready jobs
        let partition_point = partition(&mut self.scheduled_jobs, |job| job.is_ready(now));

        // Split the list in two parts: ready and unready jobs
        let (ready_jobs, unready_jobs) = self.scheduled_jobs.split_at_mut(partition_point);

        // Calculate the next evaluation time of unready jobs
        self.eval_in = unready_jobs.iter().ttl(now);

        // Return iterator over ready jobs
        JobsIterator::new(ready_jobs)
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
            self.eval_in = self.scheduled_jobs.ttl(&self.last_eval);
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
