use std::fmt::Write;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    caniot::{self, traits::TempSensType},
    controller::DeviceAlert,
    utils::{join_labels, DeviceLabel, PrometheusExporterTrait},
};

use super::{CaniotDevice, DeviceStats};

#[derive(Debug, Clone, Serialize)]
pub struct CaniotDeviceInfos {
    pub did: caniot::DeviceId,
    pub is_seen: bool,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_seen_from_now: Option<u32>, // seconds
    pub controller_attached: bool,
    pub controller_name: Option<String>,
    pub controller_display_name: Option<String>,
    pub controller_metrics: Vec<String>,
    pub stats: DeviceStats,
    pub measures: Option<caniot::BoardClassTelemetry>,

    // measures
    pub board_temperature: Option<f32>,
    pub outside_temperature: Option<f32>,
    pub board_temp_min: Option<f32>,
    pub board_temp_max: Option<f32>,
    pub board_temp_avg: Option<f32>,
    pub outside_temp_min: Option<f32>,
    pub outside_temp_max: Option<f32>,
    pub outside_temp_avg: Option<f32>,

    // current alert
    pub active_alert: Option<DeviceAlert>,

    // ui view name
    pub ui_view_name: Option<String>,
}

impl Into<CaniotDeviceInfos> for &CaniotDevice {
    fn into(self) -> CaniotDeviceInfos {
        // If controller get the controller infos
        let mut controller_display_name = None;
        let mut controller_name = None;
        let mut active_alert = None;
        let mut controller_attached = false;
        let mut ui_view_name = None;
        if let Some(controller) = &self.controller {
            controller_attached = true;

            let infos = controller.wrapper_get_infos();
            controller_name = Some(infos.name);
            controller_display_name = infos.display_name;
            active_alert = controller.wrapper_get_alert();
            ui_view_name = infos.ui_view_name;
        }

        let class_last_telemetry = self.measures.get_class_telemetry();

        CaniotDeviceInfos {
            did: self.did,
            last_seen: self.last_seen,
            controller_attached: controller_attached,
            controller_name,
            controller_display_name,
            controller_metrics: self.get_controller_metrics(),
            is_seen: self.is_seen(),
            last_seen_from_now: self.last_seen_from_now(),
            stats: self.stats,
            measures: *class_last_telemetry,
            board_temperature: class_last_telemetry
                .and_then(|m| m.get_temperature(TempSensType::BoardSensor)),
            board_temp_min: self.measures.get_board_temp_monitor().get_min().cloned(),
            board_temp_max: self.measures.get_board_temp_monitor().get_max().cloned(),
            board_temp_avg: self.measures.get_board_temp_monitor().get_avg().cloned(),
            outside_temp_min: self.measures.get_outside_temp_monitor().get_min().cloned(),
            outside_temp_max: self.measures.get_outside_temp_monitor().get_max().cloned(),
            outside_temp_avg: self.measures.get_outside_temp_monitor().get_avg().cloned(),
            outside_temperature: class_last_telemetry
                .and_then(|m| m.get_temperature(TempSensType::AnyExternal)),
            active_alert,
            ui_view_name,
        }
    }
}

impl<'a> PrometheusExporterTrait<'a> for CaniotDeviceInfos {
    type Label = DeviceLabel;
    fn export(&self, labels: impl AsRef<[&'a Self::Label]>) -> String {
        let str_labels = join_labels(&labels);
        let mut buf = String::new();

        if let Some(last_seen) = self.last_seen {
            writeln!(
                &mut buf,
                "device_last_seen {{{str_labels}}} {}",
                last_seen.timestamp()
            )
            .unwrap();
        }

        if let Some(last_seen_from_now) = self.last_seen_from_now {
            writeln!(
                &mut buf,
                "device_last_seen_from_now {{{str_labels}}} {}",
                last_seen_from_now
            )
            .unwrap();
        }

        write!(
            &mut buf,
            "device_controller_attached {{{str_labels}}} {}\n\
            device_is_seen {{{str_labels}}} {}\n\
            device_rx {{{str_labels}}} {}\n\
            device_tx {{{str_labels}}} {}\n\
            device_telemetry_rx {{{str_labels}}} {}\n\
            device_telemetry_tx {{{str_labels}}} {}\n\
            device_command_tx {{{str_labels}}} {}\n\
            device_attribute_rx {{{str_labels}}} {}\n\
            device_attribute_tx {{{str_labels}}} {}\n\
            device_err_rx {{{str_labels}}} {}\n\
            device_reset_requested {{{str_labels}}} {}\n\
            device_reset_settings_requested {{{str_labels}}} {}\n\
            device_jobs_currently_scheduled {{{str_labels}}} {}\n\
            device_jobs_processed {{{str_labels}}} {}\n",
            if self.controller_attached { 1 } else { 0 },
            if self.is_seen { 1 } else { 0 },
            self.stats.rx,
            self.stats.tx,
            self.stats.telemetry_rx,
            self.stats.telemetry_tx,
            self.stats.command_tx,
            self.stats.attribute_rx,
            self.stats.attribute_tx,
            self.stats.err_rx,
            self.stats.reset_requested,
            self.stats.reset_settings_requested,
            self.stats.jobs_currently_scheduled,
            self.stats.jobs_processed,
        )
        .unwrap();

        if let Some(board_temperature) = self.board_temperature {
            writeln!(
                &mut buf,
                "device_temperature {{{str_labels},sensor=\"embedded\"}} {}",
                board_temperature
            )
            .unwrap();
        }

        if let Some(outside_temperature) = self.outside_temperature {
            writeln!(
                &mut buf,
                "device_temperature {{{str_labels},sensor=\"external\"}} {}",
                outside_temperature
            )
            .unwrap();
        }

        for metric in &self.controller_metrics {
            writeln!(&mut buf, "{}", metric).unwrap();
        }

        buf
    }
}
