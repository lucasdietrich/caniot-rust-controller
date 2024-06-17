use std::collections::hash_map;
use std::slice::Iter;

/// Trait for objects that can expire.
pub trait ExpirableTrait<T>
where
    // TODO
    // using num_traits::Zero instead of Default is better however we can't implement
    // a foreign trait for a foreign type like Zero for std::time::Duration
    // so you need to make sure that the Default value for your type is a valid "zero" value.
    T: Eq + Ord + Default,
{
    /// Returns whether the object is expirable, i.e. has a time to expire.
    fn expirable(&self) -> bool {
        self.ttl().is_some()
    }

    /// Returns whether the object has expired.
    fn expired(&self) -> bool {
        self.ttl().map_or(false, |t| t == T::default())
    }

    /// Returns the time until the object expires (time to live)
    /// If the object is a list, it returns the minimum time to expire.
    ///
    /// # Returns
    ///
    /// Some(Duration) if the object expires, otherwise None.
    fn ttl(&self) -> Option<T> {
        None
    }

    // TODO add variant with now parameter
    // fn time_to_expire_for_now(&self, now: std::time::Instant) -> Option<Duration>;
}

impl<T, E> ExpirableTrait<T> for &E
where
    E: ExpirableTrait<T>,
    T: Eq + Ord + Default,
{
    fn ttl(&self) -> Option<T> {
        (*self).ttl()
    }
}

impl<T, E> ExpirableTrait<T> for Box<E>
where
    E: ExpirableTrait<T>,
    T: Eq + Ord + Default,
{
    fn ttl(&self) -> Option<T> {
        (**self).ttl()
    }
}

/// Implement ExpirableTrait for Iter.
impl<T, E> ExpirableTrait<T> for Iter<'_, E>
where
    E: ExpirableTrait<T>,
    T: Eq + Ord + Default,
{
    /// Returns the minimum time to live of an iterator of expirable objects.
    ///
    /// # Returns
    ///
    /// The minimum time to live of the iterator of expirable objects.
    /// If the iterator is empty, it returns None.
    fn ttl(&self) -> Option<T> {
        self.clone().filter_map(|e| e.ttl()).min()
    }
}

/// Implement ExpirableTrait for &[E].
impl<T, E> ExpirableTrait<T> for &[E]
where
    E: ExpirableTrait<T>,
    T: Eq + Ord + Default,
{
    fn ttl(&self) -> Option<T> {
        self.iter().ttl()
    }
}

/// Implement ExpirableTrait for Vec<E>.
impl<T, E> ExpirableTrait<T> for Vec<E>
where
    E: ExpirableTrait<T>,
    T: Eq + Ord + Default,
{
    fn ttl(&self) -> Option<T> {
        self.as_slice().ttl()
    }
}

impl<T, K, E> ExpirableTrait<T> for hash_map::Values<'_, K, E>
where
    E: ExpirableTrait<T>,
    T: Eq + Ord + Default,
{
    fn ttl(&self) -> Option<T> {
        self.clone().filter_map(|e| e.ttl()).min()
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
pub fn ttl<T>(results: &[Option<T>]) -> Option<T>
where
    T: Eq + Ord + Default + Copy,
{
    results.iter().filter_map(|t| *t).min()
}
