use crate::{
    controller::copro_controller::measurements::{EnergyMeterTrait, EnvironmentalTrait},
    utils::{join_labels, DeviceLabel, PrometheusExporterTrait},
};
use std::fmt::Write;

use super::device::BleDevice;

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
            self.rssi,
        )
        .unwrap();

        if let Some(battery_level) = self.battery_level {
            write!(
                &mut buf,
                "device_battery_level {{{labels}}} {}\n",
                battery_level,
            )
            .unwrap();
        }

        if let Some(battery_mv) = self.battery_mv {
            write!(
                &mut buf,
                "device_battery_voltage {{{labels}}} {}\n",
                (battery_mv as f32) / 1000.0,
            )
            .unwrap();
        }

        match &self.measurements {
            super::measurements::BleMeasurement::Environemental(meas) => {
                if let Some(temperature) = meas.temperature() {
                    write!(
                        &mut buf,
                        "device_temperature {{{labels}}} {}\n",
                        temperature,
                    )
                    .unwrap();
                }

                if let Some(humidity) = meas.humidity() {
                    write!(&mut buf, "device_humidity {{{labels}}} {}\n", humidity,).unwrap();
                }
            }
            super::measurements::BleMeasurement::EnergyMeter(meas) => {
                if let Some(power) = meas.power() {
                    write!(&mut buf, "device_power {{{labels}}} {}\n", power,).unwrap();
                }
                if let Some(current) = meas.current() {
                    write!(&mut buf, "device_current {{{labels}}} {}\n", current,).unwrap();
                }
                if let Some(energy) = meas.energy() {
                    write!(&mut buf, "device_energy {{{labels}}} {}\n", energy,).unwrap();
                }
            }
        }

        buf
    }
}
