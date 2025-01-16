use std::{sync::Arc, time::Duration};

use log::debug;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::sleep;

use crate::{
    controller::device_filtering::{DeviceFilter, FilterCriteria},
    ha::attic,
    shared::Shared,
};

#[derive(Debug, Error)]
pub enum HaError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaConfig {}

impl Default for HaConfig {
    fn default() -> Self {
        HaConfig {}
    }
}

pub struct Ha {
    shared: Arc<Shared>,
    config: HaConfig,
}

impl Ha {
    pub fn new(config: &HaConfig, shared: &Arc<Shared>) -> Self {
        Self {
            shared: shared.clone(),
            config: config.clone(),
        }
    }

    pub async fn run(mut self) -> Result<(), HaError> {
        loop {
            attic::control_attic_heaters(&self.shared).await;

            debug!("HA controller running");
            sleep(Duration::from_secs(5)).await;
        }
    }
}
