use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::{broadcast, mpsc};

use serde::Serialize;

use crate::can::CanStats;
use crate::caniot::Request as CaniotRequest;
use crate::config::AppConfig;
use crate::controller::{CaniotStats, ControllerHandle};

pub type SharedHandle = Arc<Shared>;

/// The `Shared` struct contains fields for managing shared state between asynchronous tasks.
#[derive(Debug)]
pub struct Shared {
    /// The Tokio runtime
    pub rt: Arc<Runtime>,

    /// The CAN controller handle
    pub controller_handle: Arc<ControllerHandle>,

    /// The application configuration
    pub config: AppConfig,

    /// Used to signal the asynchronous task to shutdown
    /// The task subscribes to this channel
    pub notify_shutdown: broadcast::Sender<()>,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Stats {
    pub caniot: CaniotStats,
    pub can: CanStats,
    pub server: ServerStats,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct ServerStats {}

pub fn new_context(
    rt: Arc<Runtime>,
    controller_handle: Arc<ControllerHandle>,
    config: &AppConfig,
    notify_shutdown: broadcast::Sender<()>,
) -> SharedHandle {
    Arc::new(Shared {
        rt,
        controller_handle: controller_handle,
        config: config.clone(),
        notify_shutdown,
    })
}
