use std::collections::hash_map;
use std::ops::Add;
use std::slice::Iter;

/// Trait for objects that can expire.
pub trait ExpirableTrait<D>
where
    D: Eq + Ord,
{
    const ZERO: D;
    type Instant: Add<D, Output = Self::Instant>; // Instant/timestamp type for which the duration can be added to obtain a new instant

    /// Returns whether the object is expirable, i.e. has a time to expire.
    fn is_expirable(&self, now: &Self::Instant) -> bool {
        self.ttl(now).is_some()
    }

    /// Returns whether the object has expired.
    fn is_expired(&self, now: &Self::Instant) -> bool {
        self.ttl(now).map_or(false, |exp| exp == Self::ZERO)
    }

    /// Returns the time until the object expires (time to live)
    /// If the object is a list, it returns the minimum time to expire.
    ///
    /// # Arguments
    ///
    /// * `now` - The current instant.
    ///
    /// # Returns
    ///
    /// Some(Duration) if the object expires, otherwise None.
    fn ttl(&self, now: &Self::Instant) -> Option<D>;
}

impl<D, E> ExpirableTrait<D> for &E
where
    E: ExpirableTrait<D>,
    D: Eq + Ord,
{
    const ZERO: D = E::ZERO;
    type Instant = E::Instant;

    fn ttl(&self, now: &Self::Instant) -> Option<D> {
        (*self).ttl(now)
    }
}

impl<D, E> ExpirableTrait<D> for Box<E>
where
    E: ExpirableTrait<D>,
    D: Eq + Ord,
{
    const ZERO: D = E::ZERO;
    type Instant = E::Instant;

    fn ttl(&self, now: &Self::Instant) -> Option<D> {
        self.as_ref().ttl(now)
    }
}

impl<D, E> ExpirableTrait<D> for Iter<'_, E>
where
    E: ExpirableTrait<D>,
    D: Eq + Ord,
{
    const ZERO: D = E::ZERO;
    type Instant = E::Instant;

    /// Returns the minimum time to live of an iterator of expirable objects.
    ///
    /// # Returns
    ///
    /// The minimum time to live of the iterator of expirable objects.
    /// If the iterator is empty, it returns None.
    fn ttl(&self, now: &Self::Instant) -> Option<D> {
        self.clone().filter_map(|e| e.ttl(now)).min()
    }
}

impl<D, E, K> ExpirableTrait<D> for hash_map::Values<'_, K, E>
where
    E: ExpirableTrait<D>,
    D: Eq + Ord,
{
    const ZERO: D = E::ZERO;
    type Instant = E::Instant;

    fn ttl(&self, now: &Self::Instant) -> Option<D> {
        self.clone().filter_map(|e| e.ttl(now)).min()
    }
}

impl<D, E> ExpirableTrait<D> for &[E]
where
    E: ExpirableTrait<D>,
    D: Eq + Ord,
{
    const ZERO: D = E::ZERO;
    type Instant = E::Instant;

    fn ttl(&self, now: &Self::Instant) -> Option<D> {
        self.iter().ttl(now)
    }
}

impl<D, E> ExpirableTrait<D> for Vec<E>
where
    E: ExpirableTrait<D>,
    D: Eq + Ord,
{
    const ZERO: D = E::ZERO;
    type Instant = E::Instant;

    fn ttl(&self, now: &Self::Instant) -> Option<D> {
        self.iter().ttl(now)
    }
}

/// Returns the minimum time to live of a list of expirable objects.
///
/// # Arguments
///
/// * `results` - A list of expiration results (e.g. after calling ttl() on a list of objects).
///
/// # Returns
///
/// The minimum time to live of the list of expirable objects.
/// If the list is empty, it returns None.
pub fn ttl<D>(results: &[Option<D>]) -> Option<D>
where
    D: Eq + Ord + Default + Copy,
{
    results.iter().filter_map(|t| *t).min()
}
