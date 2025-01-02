use serde::Serialize;

use crate::utils::{join_labels, DeviceLabel, PrometheusExporterTrait};

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct DeviceStats {
    pub rx: usize,
    pub tx: usize,
    pub telemetry_rx: usize,
    pub telemetry_tx: usize,
    pub command_tx: usize,
    pub attribute_rx: usize,
    pub attribute_tx: usize,
    pub err_rx: usize,

    pub reset_requested: usize,
    pub reset_settings_requested: usize,

    // jobs
    pub jobs_currently_scheduled: usize,
    pub jobs_processed: usize,
}

impl<'a> PrometheusExporterTrait<'a> for DeviceStats {
    type Label = DeviceLabel;
    fn export(&self, labels: impl AsRef<[&'a Self::Label]>) -> String {
        let labels = join_labels(labels);
        format!(
            "device_stats_rx {{{labels}}} {}\n\
            device_stats_tx {{{labels}}} {}\n\
            device_stats_telemetry_rx {{{labels}}} {}\n\
            device_stats_telemetry_tx {{{labels}}} {}\n\
            device_stats_command_tx {{{labels}}} {}\n\
            device_stats_attribute_rx {{{labels}}} {}\n\
            device_stats_attribute_tx {{{labels}}} {}\n\
            device_stats_err_rx {{{labels}}} {}\n\
            device_stats_reset_requested {{{labels}}} {}\n\
            device_stats_reset_settings_requested {{{labels}}} {}\n\
            device_stats_jobs_currently_scheduled {{{labels}}} {}\n\
            device_stats_jobs_processed {{{labels}}} {}\n",
            self.rx,
            self.tx,
            self.telemetry_rx,
            self.telemetry_tx,
            self.command_tx,
            self.attribute_rx,
            self.attribute_tx,
            self.err_rx,
            self.reset_requested,
            self.reset_settings_requested,
            self.jobs_currently_scheduled,
            self.jobs_processed,
        )
    }
}
