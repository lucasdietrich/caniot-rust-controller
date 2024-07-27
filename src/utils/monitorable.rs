use std::{fmt::Debug, ops::Deref};

pub trait MonitorableTrait: PartialEq + Eq + Clone + Debug + Default {
    fn monitor(self) -> ValueMonitor<Self> {
        ValueMonitor::init(self)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ValueMonitor<T>
where
    T: MonitorableTrait,
{
    value: T,
    updates_count: u64,
}

impl<T> ValueMonitor<T>
where
    T: MonitorableTrait,
{
    #[allow(dead_code)]
    pub fn init(value: T) -> Self {
        Self {
            value,
            updates_count: 0,
        }
    }

    pub fn update(&mut self, new_value: T) -> Option<T> {
        if self.value != new_value {
            let old_value = self.value.clone();
            self.value = new_value;
            self.updates_count += 1;
            Some(old_value)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get(&self) -> T {
        self.value.clone()
    }

    #[allow(dead_code)]
    pub fn get_updated_count(&self) -> u64 {
        self.updates_count
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> AsRef<T> for ValueMonitor<T>
where
    T: MonitorableTrait,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for ValueMonitor<T>
where
    T: MonitorableTrait,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> MonitorableTrait for T where T: PartialEq + Eq + Clone + Debug + Default {}

impl<T> Deref for ValueMonitor<T>
where
    T: MonitorableTrait,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait MonitorableResultTrait {
    fn has_changed(&self) -> bool;
    fn is_falling(&self) -> bool;
    fn is_rising(&self) -> bool;

    fn is_unchanged(&self) -> bool {
        !self.has_changed()
    }
}

impl MonitorableResultTrait for Option<bool> {
    fn has_changed(&self) -> bool {
        self.is_some()
    }

    fn is_falling(&self) -> bool {
        matches!(self, Some(true))
    }

    fn is_rising(&self) -> bool {
        matches!(self, Some(false))
    }
}

#[cfg(test)]
mod monitorable_test {
    use super::*;

    #[test]
    fn test_value_monitor() {
        let value = 42;
        let mut monitor = value.monitor();

        assert_eq!(monitor.get(), value);
        assert_eq!(monitor.get_updated_count(), 0);

        monitor.update(42);

        assert_eq!(monitor.get(), value);
        assert_eq!(monitor.get_updated_count(), 0);

        monitor.update(43);

        assert_eq!(monitor.get(), 43);
        assert_eq!(monitor.get_updated_count(), 1);

        monitor.update(44);
        monitor.update(45);

        assert_eq!(monitor.get(), 45);
        assert_eq!(monitor.get_updated_count(), 3);
    }
}
