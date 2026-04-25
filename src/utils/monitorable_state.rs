use std::{fmt::Debug, ops::Deref};

use chrono::{DateTime, Duration, Utc};

pub trait MonitorableStateTrait: PartialEq + Eq + Clone + Debug + Default {
    fn monitor(self) -> StateMonitor<Self> {
        StateMonitor::init(self)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct StateMonitor<T>
where
    T: MonitorableStateTrait,
{
    value: T,
    updates_count: u64,
}

impl<T> StateMonitor<T>
where
    T: MonitorableStateTrait,
{
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

    pub fn get(&self) -> T {
        self.value.clone()
    }

    pub fn get_updated_count(&self) -> u64 {
        self.updates_count
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> AsRef<T> for StateMonitor<T>
where
    T: MonitorableStateTrait,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for StateMonitor<T>
where
    T: MonitorableStateTrait,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> MonitorableStateTrait for T where T: PartialEq + Eq + Clone + Debug + Default {}

impl<T> Deref for StateMonitor<T>
where
    T: MonitorableStateTrait,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait MonitorableResultTrait {
    #[allow(dead_code)]
    fn has_changed(&self) -> bool;
    fn is_falling(&self) -> bool;
    fn is_rising(&self) -> bool;

    #[allow(dead_code)]
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

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TimeBoolMonitor {
    monitor: StateMonitor<bool>,
    last_updated_at: Option<DateTime<Utc>>,
    time_high: Duration,
    time_low: Duration,
}

impl TimeBoolMonitor {
    pub fn new(initial_value: bool, instant: DateTime<Utc>) -> Self {
        Self {
            monitor: StateMonitor::init(initial_value),
            last_updated_at: Some(instant),
            time_high: Duration::zero(),
            time_low: Duration::zero(),
        }
    }

    pub fn update(&mut self, new_value: bool, instant: &DateTime<Utc>) -> Option<bool> {
        let result = self.monitor.update(new_value);
        if result.has_changed() {
            let time_since_last_update = *instant - self.last_updated_at.unwrap_or(*instant);
            self.last_updated_at = Some(*instant);

            if result.is_rising() {
                self.time_low = self.time_low + time_since_last_update;
            } else {
                self.time_high = self.time_high + time_since_last_update;
            }
        }
        result
    }

    pub fn time_high_since_last_change(&self, now: &DateTime<Utc>) -> Duration {
        self.get()
            .then_some(*now - self.last_updated_at.unwrap_or(*now))
            .unwrap_or_default()
    }

    pub fn time_low_since_last_change(&self, now: &DateTime<Utc>) -> Duration {
        (!self.get())
            .then_some(*now - self.last_updated_at.unwrap_or(*now))
            .unwrap_or_default()
    }

    pub fn get(&self) -> bool {
        self.monitor.get()
    }

    pub fn get_updated_count(&self) -> u64 {
        self.monitor.get_updated_count()
    }

    pub fn get_last_updated_at(&self) -> Option<DateTime<Utc>> {
        self.last_updated_at
    }

    pub fn get_total_time_high(&self, now: &DateTime<Utc>) -> Duration {
        self.time_high + self.time_high_since_last_change(now)
    }

    pub fn get_total_time_low(&self, now: &DateTime<Utc>) -> Duration {
        self.time_low + self.time_low_since_last_change(now)
    }
}

impl Deref for TimeBoolMonitor {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        self.monitor.deref()
    }
}

#[cfg(test)]
mod monitorable_state_test {
    use super::*;

    #[test]
    fn test_state_monitor() {
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

    #[test]
    fn test_time_bool_monitor() {
        let now = Utc::now();
        let mut monitor = TimeBoolMonitor::new(false, now);

        assert_eq!(monitor.get(), false);
        assert_eq!(monitor.get_updated_count(), 0);
        assert_eq!(monitor.get_total_time_high(&now), Duration::zero());
        assert_eq!(monitor.get_total_time_low(&now), Duration::zero());

        let later = now + Duration::seconds(5);
        monitor.update(true, &later);

        assert_eq!(monitor.get(), true);
        assert_eq!(monitor.get_updated_count(), 1);
        assert_eq!(monitor.get_total_time_high(&later), Duration::zero());
        assert_eq!(monitor.get_total_time_low(&later), Duration::seconds(5));

        let even_later = later + Duration::seconds(3);
        monitor.update(false, &even_later);

        assert_eq!(monitor.get(), false);
        assert_eq!(monitor.get_updated_count(), 2);
        assert_eq!(
            monitor.get_total_time_high(&even_later),
            Duration::seconds(3)
        );
        assert_eq!(
            monitor.get_total_time_low(&even_later),
            Duration::seconds(5)
        );
    }
}
