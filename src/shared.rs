use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::{broadcast, mpsc};

use serde::Serialize;

use crate::caniot::Request as CaniotRequest;
use crate::config::AppConfig;
use crate::can::CanStats;
use crate::controller::{ControllerActorHandle, CaniotStats};

pub type SharedHandle = Arc<Shared>;

/// The `Shared` struct contains fields for managing shared state between asynchronous tasks.
#[derive(Debug)]
pub struct Shared {
    pub rt: Arc<Runtime>,

    pub controller_actor_handle: Arc<ControllerActorHandle>,

    /// The application configuration
    pub config: AppConfig,

    /// Used to signal the asynchronous task to shutdown
    /// The task subscribes to this channel
    pub notify_shutdown: broadcast::Sender<()>,

    // Message queue for sending CANIOT commands to the CAN bus
    // pub can_tx_queue: mpsc::Sender<CaniotRequest>,
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
    controller_actor_handle: Arc<ControllerActorHandle>,
    config: &AppConfig,
    notify_shutdown: broadcast::Sender<()>,
) -> SharedHandle {
    Arc::new(Shared {
        rt,
        controller_actor_handle: controller_actor_handle.clone(),
        config: config.clone(),
        notify_shutdown,
    })
}
