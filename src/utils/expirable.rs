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

// Time to Result
pub struct ExpirableTTLResults<'a, T>(&'a [Option<T>])
where
    T: Eq + Ord + Default;

impl<'a, T> ExpirableTTLResults<'a, T>
where
    T: Eq + Ord + Default,
{
    pub fn new(results: &'a [Option<T>]) -> Self {
        Self(results)
    }
}

impl<T> ExpirableTrait<T> for ExpirableTTLResults<'_, T>
where
    // Note that Copy trait is required
    T: Eq + Ord + Default + Copy,
{
    fn ttl(&self) -> Option<T> {
        self.0.iter().filter_map(|t| *t).min()
    }
}

impl<'a, T> From<&'a [Option<T>]> for ExpirableTTLResults<'a, T>
where
    T: Eq + Ord + Default,
{
    fn from(results: &'a [Option<T>]) -> Self {
        Self::new(results)
    }
}
