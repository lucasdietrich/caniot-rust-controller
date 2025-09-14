use chrono::{DateTime, Utc};
use rocket::time::Time;

use crate::utils::monitorable_measure::ValueMonitor;

pub trait RssiTrait {
    fn rssi(&self) -> i8;
}

pub trait BatteryTrait {
    fn battery_mv(&self) -> Option<u16>;
    fn battery_level(&self) -> Option<u8>;
}

pub trait TimestampedTrait {
    fn timestamp(&self) -> Option<DateTime<Utc>>;
}

pub trait EnvironmentalTrait {
    fn temperature(&self) -> Option<f32>;
    fn humidity(&self) -> Option<f32>;
}

pub trait EnvironmentalMinMaxTrait {
    fn temperature_min(&self) -> Option<f32>;
    fn temperature_max(&self) -> Option<f32>;
    fn humidity_min(&self) -> Option<f32>;
    fn humidity_max(&self) -> Option<f32>;
}

pub trait EnergyMeterTrait {
    fn power(&self) -> Option<f32>;
    fn energy(&self) -> Option<f32>;
    fn current(&self) -> Option<f32>;
}

pub trait EnergyMeterMinMaxTrait {
    fn power_min(&self) -> Option<f32>;
    fn power_max(&self) -> Option<f32>;
    fn current_min(&self) -> Option<f32>;
    fn current_max(&self) -> Option<f32>;
}

pub trait ResetableMinMaxTrait {
    fn reset_minmax(&mut self);
}

#[derive(Debug, Clone)]
pub enum BleMeasurement {
    Environemental(BleEnvironementalMeasurement),
    EnergyMeter(BleEnergyMeterMeasurement),
}

impl BleMeasurement {
    pub fn update_environemental<T: EnvironmentalTrait + TimestampedTrait>(
        &mut self,
        val: T,
    ) -> Result<(), T> {
        match self {
            BleMeasurement::Environemental(current) => {
                current.update(val);
                Ok(())
            }
            _ => Err(val),
        }
    }

    pub fn update_energy_meter<T: EnergyMeterTrait + TimestampedTrait>(
        &mut self,
        val: T,
    ) -> Result<(), T> {
        match self {
            BleMeasurement::EnergyMeter(current) => {
                current.update(val);
                Ok(())
            }
            _ => Err(val),
        }
    }
}

impl From<BleEnvironementalMeasurement> for BleMeasurement {
    fn from(measurement: BleEnvironementalMeasurement) -> Self {
        BleMeasurement::Environemental(measurement)
    }
}

impl From<BleEnergyMeterMeasurement> for BleMeasurement {
    fn from(measurement: BleEnergyMeterMeasurement) -> Self {
        BleMeasurement::EnergyMeter(measurement)
    }
}

impl TimestampedTrait for BleMeasurement {
    fn timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            BleMeasurement::Environemental(meas) => meas.timestamp(),
            BleMeasurement::EnergyMeter(meas) => meas.timestamp(),
        }
    }
}

impl ResetableMinMaxTrait for BleMeasurement {
    fn reset_minmax(&mut self) {
        match self {
            BleMeasurement::Environemental(meas) => meas.reset_minmax(),
            BleMeasurement::EnergyMeter(meas) => meas.reset_minmax(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BleEnvironementalMeasurement {
    pub timestamp: Option<DateTime<Utc>>,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub temperature_monitor: ValueMonitor<f32>,
    pub humidity_monitor: ValueMonitor<f32>,
}

impl BleEnvironementalMeasurement {
    pub fn init<T: TimestampedTrait + EnvironmentalTrait>(val: T) -> Self {
        let temperature = val.temperature();
        let humidity = val.humidity();

        Self {
            timestamp: val.timestamp(),
            temperature,
            humidity,
            temperature_monitor: ValueMonitor::init(temperature),
            humidity_monitor: ValueMonitor::init(humidity),
        }
    }

    pub fn update<T: TimestampedTrait + EnvironmentalTrait>(&mut self, val: T) {
        self.timestamp = val.timestamp();
        self.temperature = val.temperature();
        self.humidity = val.humidity();

        if let Some(temp) = self.temperature {
            self.temperature_monitor.update(&temp);
        }

        if let Some(humidity) = self.humidity {
            self.humidity_monitor.update(&humidity);
        }
    }
}

impl TimestampedTrait for BleEnvironementalMeasurement {
    fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }
}

impl EnvironmentalTrait for BleEnvironementalMeasurement {
    fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    fn humidity(&self) -> Option<f32> {
        self.humidity
    }
}

impl EnvironmentalMinMaxTrait for BleEnvironementalMeasurement {
    fn temperature_min(&self) -> Option<f32> {
        self.temperature_monitor.get_min().cloned()
    }

    fn temperature_max(&self) -> Option<f32> {
        self.temperature_monitor.get_max().cloned()
    }

    fn humidity_min(&self) -> Option<f32> {
        self.humidity_monitor.get_min().cloned()
    }

    fn humidity_max(&self) -> Option<f32> {
        self.humidity_monitor.get_max().cloned()
    }
}

impl ResetableMinMaxTrait for BleEnvironementalMeasurement {
    fn reset_minmax(&mut self) {
        self.temperature_monitor.reset();
        self.humidity_monitor.reset();
    }
}

#[derive(Debug, Clone)]
pub struct BleEnergyMeterMeasurement {
    pub timestamp: Option<DateTime<Utc>>,
    pub power: Option<f32>,
    pub energy: Option<f32>,
    pub current: Option<f32>,
    pub power_monitor: ValueMonitor<f32>,
    pub current_monitor: ValueMonitor<f32>,
}

impl BleEnergyMeterMeasurement {
    pub fn init<T: TimestampedTrait + EnergyMeterTrait>(val: T) -> Self {
        Self {
            timestamp: val.timestamp(),
            power: val.power(),
            energy: val.energy(),
            current: val.current(),
            power_monitor: ValueMonitor::init(val.power()),
            current_monitor: ValueMonitor::init(val.current()),
        }
    }

    pub fn update<T: TimestampedTrait + EnergyMeterTrait>(&mut self, val: T) {
        self.timestamp = val.timestamp();
        self.energy = val.energy();
        self.power = val.power();
        self.current = val.current();

        if let Some(power) = self.power {
            self.power_monitor.update(&power);
        }
        if let Some(current) = self.current {
            self.current_monitor.update(&current);
        }
    }
}

impl TimestampedTrait for BleEnergyMeterMeasurement {
    fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }
}

impl EnergyMeterTrait for BleEnergyMeterMeasurement {
    fn power(&self) -> Option<f32> {
        self.power
    }

    fn energy(&self) -> Option<f32> {
        self.energy
    }

    fn current(&self) -> Option<f32> {
        self.current
    }
}

impl EnergyMeterMinMaxTrait for BleEnergyMeterMeasurement {
    fn power_min(&self) -> Option<f32> {
        self.power_monitor.get_min().cloned()
    }

    fn power_max(&self) -> Option<f32> {
        self.power_monitor.get_max().cloned()
    }

    fn current_min(&self) -> Option<f32> {
        self.current_monitor.get_min().cloned()
    }

    fn current_max(&self) -> Option<f32> {
        self.current_monitor.get_max().cloned()
    }
}

impl ResetableMinMaxTrait for BleEnergyMeterMeasurement {
    fn reset_minmax(&mut self) {
        self.power_monitor.reset();
        self.current_monitor.reset();
    }
}
