use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};

use serde::Serialize;

use crate::caniot::Request as CaniotRequest;
use crate::config::AppConfig;

pub type SharedHandle = Arc<Shared>;

/// The `Shared` struct contains fields for managing shared state between asynchronous tasks.
#[derive(Debug)]
pub struct Shared {
    /// The application configuration
    pub config: AppConfig,

    // TODO ???
    // The Tokio runtime. Some tasks may need to spawn additional tasks onto the runtime.
    // pub rt: Mutex<tokio::runtime::Runtime>,

    /// Gather all statistics about the application
    pub stats: Mutex<Stats>,

    /// Used to signal the asynchronous task to shutdown
    /// The task subscribes to this channel
    pub notify_shutdown: broadcast::Sender<()>,

    // TODO ???
    // /// Used to signal the asynchronous task has completed
    // /// The task drops the sender when it shuts down
    // pub _shutdown_complete: mpsc::Sender<()>,
    /// Message queue for sending CANIOT commands to the CAN bus
    pub can_tx_queue: mpsc::Sender<CaniotRequest>,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Stats {
    pub can: CanStats,
    pub server: ServerStats,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct CanStats {
    pub rx: usize,
    pub tx: usize,
    pub err: usize,
    pub malformed: usize,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct ServerStats {}

pub fn new_context(
    config: AppConfig,
    notify_shutdown: broadcast::Sender<()>,
    can_tx_queue: mpsc::Sender<CaniotRequest>,
) -> SharedHandle {
    Arc::new(Shared {
        config,
        stats: Mutex::new(Stats {
            can: CanStats {
                rx: 0,
                tx: 0,
                err: 0,
                malformed: 0,
            },
            server: ServerStats {},
        }),
        notify_shutdown,
        can_tx_queue,
    })
}
