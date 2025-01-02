use crate::utils::{join_labels, DeviceLabel, PrometheusExporterTrait};
use std::fmt::Write;

use super::device::{BleDevice, Stats};

impl<'a> PrometheusExporterTrait<'a> for BleDevice {
    type Label = DeviceLabel;

    fn export(&self, labels: impl AsRef<[&'a Self::Label]>) -> String {
        let labels = join_labels(labels);

        let mut buf = String::new();

        write!(
            &mut buf,
            "device_is_seen {{{labels}}} {}\n\
            device_last_seen {{{labels}}} {}\n\
            device_last_seen_from_now {{{labels}}} {}\n\
            device_stats_rx {{{labels}}} {}\n\
            device_rssi {{{labels}}} {}\n\
            ",
            1,
            self.last_seen.timestamp(),
            self.last_seen_from_now(),
            self.stats.rx_packets,
            self.last_measurement.rssi,
        )
        .unwrap();

        if let Some(temperature) = self.last_measurement.temperature {
            write!(
                &mut buf,
                "device_temperature {{{labels}}} {}\n",
                temperature,
            )
            .unwrap();
        }

        if let Some(humidity) = self.last_measurement.humidity {
            write!(&mut buf, "device_humidity {{{labels}}} {}\n", humidity,).unwrap();
        }

        if let Some(battery_level) = self.last_measurement.battery_level {
            write!(
                &mut buf,
                "device_battery_level {{{labels}}} {}\n",
                battery_level,
            )
            .unwrap();
        }

        if let Some(battery_mv) = self.last_measurement.battery_mv {
            write!(
                &mut buf,
                "device_battery_voltage {{{labels}}} {}\n",
                (battery_mv as f32) / 1000.0,
            )
            .unwrap();
        }

        buf
    }
}
