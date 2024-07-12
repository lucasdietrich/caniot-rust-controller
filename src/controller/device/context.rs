use chrono::{DateTime, Utc};

use super::DevCtrlSchedJobTrait;

#[derive(Default, Debug)]
pub struct ProcessContext {
    // Received frame timestamp
    pub frame_received_at: DateTime<Utc>,

    // New jobs to be scheduled
    pub new_jobs: Vec<Box<dyn DevCtrlSchedJobTrait>>,
}

impl ProcessContext {
    pub fn new(received_at: DateTime<Utc>) -> Self {
        ProcessContext {
            frame_received_at: received_at,
            new_jobs: vec![],
            ..Default::default()
        }
    }

    pub fn add_job<J>(&mut self, job: J)
    where
        J: DevCtrlSchedJobTrait,
    {
        self.new_jobs.push(Box::new(job));
    }
}
