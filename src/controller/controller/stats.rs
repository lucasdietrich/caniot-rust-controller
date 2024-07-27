use serde::Serialize;

use crate::utils::{PrometheusExporterTrait, PrometheusNoLabel};

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct ControllerStats {
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

    // Internals
    pub api_rx: usize,  // Internal API calls
    pub loop_runs: u64, // Number of times the controller loop has been executed
}

impl<'a> PrometheusExporterTrait<'a> for ControllerStats {
    type Label = PrometheusNoLabel;

    fn export(&self, _labels: impl AsRef<[&'a Self::Label]>) -> String {
        format!(
            "controller_iface_rx {}\n\
            controller_iface_tx {}\n\
            controller_iface_err {}\n\
            controller_iface_malformed {}\n\
            controller_broadcast_tx {}\n\
            controller_pq_pushed {}\n\
            controller_pq_timeout {}\n\
            controller_pq_answered {}\n\
            controller_pq_duplicate_dropped {}\n\
            controller_api_rx {}\n\
            controller_loop_runs {}\n",
            self.iface_rx,
            self.iface_tx,
            self.iface_err,
            self.iface_malformed,
            self.broadcast_tx,
            self.pq_pushed,
            self.pq_timeout,
            self.pq_answered,
            self.pq_duplicate_dropped,
            self.api_rx,
            self.loop_runs
        )
    }
}
