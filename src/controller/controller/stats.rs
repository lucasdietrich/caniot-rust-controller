use serde::Serialize;

use crate::{
    bus::CanStats,
    utils::{PrometheusExporterTrait, PrometheusNoLabel},
};

use super::controller::ControllerCoreStats;

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

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct ControllerStats {
    pub caniot: CaniotControllerStats,
    pub core: ControllerCoreStats,
    pub can: CanStats,
}

impl<'a> PrometheusExporterTrait<'a> for ControllerStats {
    type Label = PrometheusNoLabel;

    fn export(&self, _labels: impl AsRef<[&'a Self::Label]>) -> String {
        format!(
            "controller_caniot_iface_rx {}\n\
            controller_caniot_iface_tx {}\n\
            controller_caniot_iface_err {}\n\
            controller_caniot_iface_malformed {}\n\
            controller_caniot_broadcast_tx {}\n\
            controller_caniot_pq_pushed {}\n\
            controller_caniot_pq_timeout {}\n\
            controller_caniot_pq_answered {}\n\
            controller_caniot_pq_duplicate_dropped {}\n\
            controller_api_rx {}\n\
            controller_loop_runs {}\n\
            bus_can_rx {}\n\
            bus_can_tx {}\n\
            bus_can_err {}\n\
            bus_can_unhandled {}\n",
            self.caniot.iface_rx,
            self.caniot.iface_tx,
            self.caniot.iface_err,
            self.caniot.iface_malformed,
            self.caniot.broadcast_tx,
            self.caniot.pq_pushed,
            self.caniot.pq_timeout,
            self.caniot.pq_answered,
            self.caniot.pq_duplicate_dropped,
            self.core.api_rx,
            self.core.loop_runs,
            self.can.rx,
            self.can.tx,
            self.can.err,
            self.can.unhandled,
        )
    }
}
