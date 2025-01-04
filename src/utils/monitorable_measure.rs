use std::{fmt::Debug, ops::Deref};

use crate::caniot::error;

pub trait MonitorableValueTrait: PartialOrd + Clone + Debug + Default {
    fn monitor(self) -> ValueMonitor<Self> {
        ValueMonitor::init(Some(self))
    }
}

#[derive(Clone, Debug, Default)]
pub struct ValueMonitor<T>
where
    T: MonitorableValueTrait,
{
    min: Option<T>,
    max: Option<T>,
}

impl<T> ValueMonitor<T>
where
    T: MonitorableValueTrait,
{
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
        }
    }

    pub fn init(value: Option<T>) -> Self {
        Self {
            min: value.clone(),
            max: value.clone(),
        }
    }

    pub fn update(&mut self, new_value: &T) {
        match self.min {
            Some(ref min) if new_value < min => self.min = Some(new_value.clone()),
            None => self.min = Some(new_value.clone()),
            _ => {}
        }

        match self.max {
            Some(ref max) if new_value > max => self.max = Some(new_value.clone()),
            None => self.max = Some(new_value.clone()),
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.min = None;
        self.max = None;
    }

    pub fn get_min(&self) -> Option<&T> {
        self.min.as_ref()
    }

    pub fn get_max(&self) -> Option<&T> {
        self.max.as_ref()
    }

    pub fn get_avg(&self) -> Option<&T> {
        // error!("ValueMonitor::get_avg not implemented");
        None
    }
}

impl<T> MonitorableValueTrait for T where T: PartialOrd + Clone + Debug + Default {}

#[cfg(test)]
mod monitorable_measure_test {
    use super::*;

    #[test]
    fn test_value_monitor() {
        let value = 42;
        let mut monitor = value.monitor();

        assert_eq!(monitor.get_min(), Some(&42));
        assert_eq!(monitor.get_max(), Some(&42));

        monitor.update(&24);
        assert_eq!(monitor.get_min(), Some(&24));
        assert_eq!(monitor.get_max(), Some(&42));

        monitor.update(&84);
        assert_eq!(monitor.get_min(), Some(&24));
        assert_eq!(monitor.get_max(), Some(&84));
    }
}
