use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use chrono::{DateTime, Utc};

use crate::{
    caniot::Attribute,
    database::{SettingsStore, Storage},
};

use super::{DeviceError, JobTrait};

pub struct ProcessContext<'f> {
    // Received frame timestamp
    pub frame_received_at: Option<DateTime<Utc>>,

    // // Current UTC time
    // pub current_utc_time: DateTime<Utc>,

    // New jobs to be scheduled
    pub new_jobs: Vec<Box<dyn JobTrait>>,

    // Request jobs update
    pub request_jobs_update: bool,

    // Settings store
    pub storage: Arc<Storage>,

    // Update attributes
    pub update_attributes: HashMap<Attribute, u32>,

    // Storage update future
    //
    // let future = Box::pin(async move {
    //    let storage = storage.clone();
    //    loop {
    //        ...
    //    }
    // });
    pub storage_update_future:
        Option<Pin<Box<dyn Future<Output = Result<(), DeviceError>> + Send + 'f>>>,
}

impl<'f> ProcessContext<'f> {
    pub fn new(received_at: Option<DateTime<Utc>>, storage: Arc<Storage>) -> Self {
        ProcessContext {
            frame_received_at: received_at,
            new_jobs: vec![],
            request_jobs_update: false,
            storage,
            update_attributes: HashMap::new(),
            storage_update_future: None,
        }
    }

    pub fn add_job<J>(&mut self, job: J)
    where
        J: JobTrait,
    {
        self.new_jobs.push(Box::new(job));
    }

    pub fn request_jobs_update(&mut self) {
        self.request_jobs_update = true;
    }

    pub fn get_settings_store<'s>(&'s self) -> SettingsStore<'s> {
        self.storage.get_settings_store()
    }

    pub fn set_async_future<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), DeviceError>> + Send + 'f,
    {
        self.storage_update_future = Some(Box::pin(future));
    }

    pub async fn run_async_future(&mut self) -> Result<(), DeviceError> {
        if let Some(future) = self.storage_update_future.take() {
            future.await
        } else {
            Ok(())
        }
    }
}
