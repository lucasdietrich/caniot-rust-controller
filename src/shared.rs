use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};

use serde::Serialize;

pub type SharedHandle = Arc<Shared>;

/// The `Shared` struct contains fields for managing shared state between asynchronous tasks.
#[derive(Debug)]
pub struct Shared {
    // TODO ???
    // The Tokio runtime. Some tasks may need to spawn additional tasks onto the runtime.
    // pub rt: Mutex<tokio::runtime::Runtime>,
    /// Gather all statistics about the application
    pub stats: Mutex<Stats>,

    /// Used to signal the asynchronous task to shutdown
    /// The task subscribes to this channel
    pub notify_shutdown: broadcast::Sender<()>, // TODO ???
                                                // /// Used to signal the asynchronous task has completed
                                                // /// The task drops the sender when it shuts down
                                                // pub _shutdown_complete: mpsc::Sender<()>,
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

pub fn new_context(notify_shutdown: broadcast::Sender<()>) -> SharedHandle {
    Arc::new(Shared {
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
    })
}
