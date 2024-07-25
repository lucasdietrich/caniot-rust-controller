use std::fmt::Debug;

use as_any::AsAny;
use chrono::{DateTime, Duration, Utc};
use dyn_clone::DynClone;
use log::debug;

use crate::utils::{expirable::ExpirableTrait, Scheduling};

pub trait DevCtrlSchedJobTrait: AsAny + Send + Debug + DynClone {
    fn get_scheduling(&self) -> Scheduling {
        Scheduling::Unscheduled
    }
}

// YAYE! More dirty tricks to make bad code work
// TODO review this shitty Job implementation
dyn_clone::clone_trait_object!(DevCtrlSchedJobTrait);

// Implement the trait for the unit type to avoid having a job for device
// controllers that don't need any
impl DevCtrlSchedJobTrait for () {}

#[derive(Debug)]
pub enum DeviceJobImpl<'a, J>
where
    J: DevCtrlSchedJobTrait,
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
    J: DevCtrlSchedJobTrait,
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

#[derive(Debug, Clone)]
pub enum DeviceJobWrapper {
    DeviceAdd,
    DeviceRemove,
    Scheduled(Box<dyn DevCtrlSchedJobTrait>),
}

impl DeviceJobWrapper {
    pub fn get_scheduling(&self) -> Scheduling {
        match self {
            DeviceJobWrapper::DeviceAdd => Scheduling::Immediate,
            DeviceJobWrapper::DeviceRemove => Scheduling::Immediate,
            DeviceJobWrapper::Scheduled(job) => job.get_scheduling(),
        }
    }
}

impl ExpirableTrait<Duration> for DeviceJobWrapper {
    const ZERO: Duration = Duration::zero();
    type Instant = DateTime<Utc>;

    fn ttl(&self, now: &DateTime<Utc>) -> Option<Duration> {
        self.get_scheduling().time_to_next(now)
    }
}

#[derive(Debug)]
pub struct DeviceJobsContext {
    definitions: Vec<DeviceJobWrapper>,
    last_eval: DateTime<Utc>,
    eval_in: Option<Duration>,
    pending: Vec<TriggeredDeviceJob>,
}

impl DeviceJobsContext {
    pub fn new(first_eval: DateTime<Utc>) -> Self {
        // Default jobs
        let init_jobs = vec![DeviceJobWrapper::DeviceAdd];
        let init_eval_in = init_jobs.ttl(&first_eval);

        Self {
            last_eval: first_eval,
            eval_in: init_eval_in,
            definitions: init_jobs,
            pending: vec![],
        }
    }

    pub fn register_new_jobs(&mut self, jobs_definitions: Vec<Box<dyn DevCtrlSchedJobTrait>>) {
        let new_definitions: Vec<DeviceJobWrapper> = jobs_definitions
            .into_iter()
            .map(|job| DeviceJobWrapper::Scheduled(job))
            .collect();

        // Perform the registration + calculation if the jobs changed
        if !new_definitions.is_empty() {
            debug!("Registering new jobs: {:?}", new_definitions);
            self.definitions.extend(new_definitions);
            self.eval_in = self.definitions.ttl(&self.last_eval);
        }
    }

    pub fn shift(&mut self, now: &DateTime<Utc>) {
        self.definitions.retain(|definition| {
            let scheduling = definition.get_scheduling();
            let triggered_jobs: Vec<TriggeredDeviceJob> = scheduling
                .occurences(&self.last_eval, now)
                .into_iter()
                .map(|dt| TriggeredDeviceJob::new(dt, definition.clone()))
                .collect();
            self.pending.extend(triggered_jobs);

            !scheduling.into_next().is_unescheduled()
        });

        // Update the last evaluation time
        self.last_eval = *now;

        // Update the next evaluation time
        self.eval_in = self.definitions.ttl(&self.last_eval);
    }

    pub fn pop_pending(&mut self) -> Option<TriggeredDeviceJob> {
        self.pending.pop()
    }

    pub fn get_jobs_count(&self) -> usize {
        self.definitions.len()
    }
}

impl ExpirableTrait<Duration> for DeviceJobsContext {
    const ZERO: Duration = Duration::zero();
    type Instant = DateTime<Utc>;

    fn ttl(&self, now: &DateTime<Utc>) -> Option<Duration> {
        if !self.pending.is_empty() {
            Some(Self::ZERO)
        } else if let Some(time_to_eval) = self.eval_in {
            if self.last_eval + time_to_eval <= *now {
                Some(Self::ZERO)
            } else {
                Some(self.last_eval + time_to_eval - *now)
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TriggeredDeviceJob {
    pub timestamp: DateTime<Utc>,
    pub definition: DeviceJobWrapper,
}

impl TriggeredDeviceJob {
    pub fn new(timestamp: DateTime<Utc>, definition: DeviceJobWrapper) -> Self {
        Self {
            timestamp,
            definition,
        }
    }
}
