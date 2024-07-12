use chrono::{Duration, NaiveDateTime, Utc};

use crate::utils::expirable::ttl;

use super::expirable::ExpirableTrait;

#[derive(Debug, Default)]
struct DurExpirable {
    ttl: Option<u64>,
}

impl ExpirableTrait<u64> for DurExpirable {
    const ZERO: u64 = 0;
    type Instant = u64;

    fn ttl(&self, _now: &u64) -> Option<u64> {
        self.ttl
    }
}

#[derive(Debug, Default)]
struct InstExpirable {
    expiration_instant: Option<u64>,
}

impl ExpirableTrait<u64> for InstExpirable {
    const ZERO: u64 = 0;
    type Instant = u64;

    fn ttl(&self, now: &u64) -> Option<u64> {
        self.expiration_instant
            .and_then(|exp| if exp > *now { Some(exp - now) } else { Some(0) })
    }
}

#[test]
fn test_duration_expirable() {
    let now = 1;

    let expired = DurExpirable { ttl: Some(0) };
    assert!(expired.is_expirable(&now));
    assert!(expired.is_expired(&now));

    let not_expired = DurExpirable { ttl: Some(1) };
    assert!(not_expired.is_expirable(&now));
    assert!(!not_expired.is_expired(&now));

    let not_expirable = DurExpirable { ttl: None };
    assert!(!not_expirable.is_expirable(&now));
    assert!(!not_expirable.is_expired(&now));
}

#[test]
fn test_instant_expirable() {
    let now = 1;

    let expired = InstExpirable {
        expiration_instant: Some(0),
    };
    assert!(expired.is_expirable(&now));
    assert!(expired.is_expired(&now));

    let not_expired = InstExpirable {
        expiration_instant: Some(1),
    };
    assert!(not_expired.is_expirable(&now));
    assert!(not_expired.is_expired(&now));

    let expired = InstExpirable {
        expiration_instant: Some(2),
    };
    assert!(expired.is_expirable(&now));
    assert!(!expired.is_expired(&now));

    let not_expirable = InstExpirable {
        expiration_instant: None,
    };
    assert!(!not_expirable.is_expirable(&now));
    assert!(!not_expirable.is_expired(&now));
}

#[test]
fn test_iter_expirable() {
    fn create_vec(durations: &[i64]) -> Vec<DurExpirable> {
        durations
            .iter()
            .map(|d| {
                let ttl = if *d < 0 { None } else { Some(*d as u64) };
                DurExpirable { ttl }
            })
            .collect()
    }

    fn test_vec(durations: &[i64], expirable: bool, expired: bool, time_to_expire: Option<u64>) {
        let now = 0;
        let vec = create_vec(durations);
        assert_eq!(vec.iter().is_expirable(&now), expirable);
        assert_eq!(vec.iter().is_expired(&now), expired);
        assert_eq!(vec.iter().ttl(&now), time_to_expire);
    }

    // negative ttl means no expiration (transformed to None)
    test_vec(&[], false, false, None);
    test_vec(&[0], true, true, Some(0));
    test_vec(&[1], true, false, Some(1));
    test_vec(&[-1], false, false, None);

    test_vec(&[0, 1], true, true, Some(0));
    test_vec(&[1, 0], true, true, Some(0));
    test_vec(&[1, 1], true, false, Some(1));
    test_vec(&[0, 0], true, true, Some(0));
    test_vec(&[0, -1], true, true, Some(0));
    test_vec(&[1, -1], true, false, Some(1));
    test_vec(&[-1, -1], false, false, None);

    test_vec(&[2, 4, 3], true, false, Some(2));
    test_vec(&[4, 2, 3], true, false, Some(2));
    test_vec(&[4, 3, 2], true, false, Some(2));

    test_vec(&[2, 4, -1], true, false, Some(2));
    test_vec(&[4, 2, -1], true, false, Some(2));
    test_vec(&[4, -1, 2], true, false, Some(2));
    test_vec(&[-1, 4, 2], true, false, Some(2));

    test_vec(&[2, 0, -1], true, true, Some(0));
    test_vec(&[0, 2, -1], true, true, Some(0));
    test_vec(&[0, -1, 2], true, true, Some(0));
    test_vec(&[-1, 0, 2], true, true, Some(0));
}

#[test]
fn test_result_ttl() {
    assert_eq!(ttl(&[Some(1), Some(2), None]), Some(1));
    assert_eq!(ttl(&[Some(2), Some(1)]), Some(1));
    assert_eq!(ttl(&[Some(2), None]), Some(2));
    assert_eq!(ttl(&[None, Some(2)]), Some(2));
    assert_eq!(ttl::<Option<u64>>(&[None, None]), None);
}

pub struct ChronoExpirable {
    expiration_instant: Option<NaiveDateTime>,
}

impl ExpirableTrait<Duration> for ChronoExpirable {
    const ZERO: Duration = Duration::zero();
    type Instant = NaiveDateTime;

    fn ttl(&self, now: &NaiveDateTime) -> Option<Duration> {
        self.expiration_instant.and_then(|exp| {
            if exp > *now {
                Some(exp - *now)
            } else {
                Some(Duration::zero())
            }
        })
    }
}

#[test]
fn test_chrono_expirable() {
    let now = Utc::now().naive_utc();
    let now_1s = now + Duration::seconds(1);
    let now_1m = now + Duration::minutes(1);
    let now_1h = now + Duration::hours(1);
    let now_1j = now + Duration::days(1);
}

pub struct StdExpirable {
    expiration_instant: Option<std::time::Instant>,
}

impl ExpirableTrait<std::time::Duration> for StdExpirable {
    const ZERO: std::time::Duration = std::time::Duration::ZERO;
    type Instant = std::time::Instant;

    fn ttl(&self, now: &std::time::Instant) -> Option<std::time::Duration> {
        self.expiration_instant.and_then(|exp| {
            if exp > *now {
                Some(exp - *now)
            } else {
                Some(std::time::Duration::ZERO)
            }
        })
    }
}
