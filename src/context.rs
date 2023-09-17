use std::sync::{Arc, Mutex};

use serde::Serialize;

pub type ContextHandle = Arc<Mutex<Context>>;

#[derive(Serialize, Debug)]
pub struct Context {
    pub stats: Stats,
    pub server: ServerStats,
}

#[derive(Serialize, Debug)]
pub struct Stats {
    pub can: CanStats,
}

#[derive(Serialize, Debug)]
pub struct CanStats {
    pub rx: usize,
    pub tx: usize,
    pub err: usize,
}

#[derive(Serialize, Debug)]
pub struct ServerStats {
    
}

pub fn new_context() -> ContextHandle {
    Arc::new(Mutex::new(Context {
        stats: Stats {
            can: CanStats {
                rx: 0,
                tx: 0,
                err: 0,
            },
        },
        server: ServerStats {},
    }))
}