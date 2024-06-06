use super::expirable::{ExpirableTTLResults, ExpirableTrait};

#[derive(Debug, Default)]
struct Expirable {
    ttl: Option<u64>,
}

impl ExpirableTrait<u64> for Expirable {
    fn ttl(&self) -> Option<u64> {
        self.ttl
    }
}

#[test]
fn test_expirable() {
    let expired = Expirable { ttl: Some(0) };
    assert!(expired.expirable());
    assert!(expired.expired());

    let not_expired = Expirable { ttl: Some(1) };
    assert!(not_expired.expirable());
    assert!(!not_expired.expired());

    let not_expirable = Expirable { ttl: None };
    assert!(!not_expirable.expirable());
    assert!(!not_expirable.expired());
}

#[test]
fn test_iter_expirable() {
    fn create_vec(durations: &[i64]) -> Vec<Expirable> {
        durations
            .iter()
            .map(|d| {
                let ttl = if *d < 0 { None } else { Some(*d as u64) };
                Expirable { ttl }
            })
            .collect()
    }

    fn test_vec(durations: &[i64], expirable: bool, expired: bool, time_to_expire: Option<u64>) {
        let vec = create_vec(durations);
        assert_eq!(vec.iter().expirable(), expirable);
        assert_eq!(vec.iter().expired(), expired);
        assert_eq!(vec.iter().ttl(), time_to_expire);
    }

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
fn test_expirable_result() {
    let inner = &[Some(1), Some(2), None];
    let result = ExpirableTTLResults::new(inner);

    assert_eq!(result.expirable(), true);
    assert_eq!(result.expired(), false);
    assert_eq!(result.ttl(), Some(1));

    let inner = &[Some(2), None];
    let result = ExpirableTTLResults::new(inner);

    assert_eq!(result.expirable(), true);
    assert_eq!(result.expired(), false);
    assert_eq!(result.ttl(), Some(2));

    let inner = &[None, None];
    let result = ExpirableTTLResults::<u64>::new(inner);

    assert_eq!(result.expirable(), false);
    assert_eq!(result.expired(), false);
    assert_eq!(result.ttl(), None);
}
