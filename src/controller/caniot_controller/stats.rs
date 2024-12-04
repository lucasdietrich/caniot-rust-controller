use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct CaniotControllerStats {
    // can interface
    pub iface_rx: usize,
    pub iface_tx: usize,
    pub iface_err: usize,
    pub iface_malformed: usize,
    // dropped ?

    // caniot broadcast
    pub broadcast_tx: usize,

    // Pending queries
    pub pq_pushed: usize,
    pub pq_timeout: usize,
    pub pq_answered: usize,
    pub pq_duplicate_dropped: usize,
}
