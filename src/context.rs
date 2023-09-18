use std::sync::{Arc, Mutex};

use serde::Serialize;

pub type ContextHandle = Arc<Mutex<Context>>;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Context {
    pub stats: Stats,
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

pub fn new_context() -> ContextHandle {
    Arc::new(Mutex::new(Context {
        stats: Stats {
            can: CanStats {
                rx: 0,
                tx: 0,
                err: 0,
                malformed: 0,
            },
            server: ServerStats {},
        },
    }))
}
