use std::collections::VecDeque;

use sorted_vec::SortedVec;

use super::expirable::ExpirableTrait;

pub struct ExpirableQueue<T, E>
where
    T: Eq + Ord + Default,
    E: ExpirableTrait<T> + Ord,
{
    queue: SortedVec<E>,
    _marker: std::marker::PhantomData<T>,
}

impl<T, E> ExpirableQueue<T, E>
where
    T: Eq + Ord + Default,
    E: ExpirableTrait<T> + Ord,
{
    pub fn new() -> Self {
        Self {
            queue: SortedVec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, item: E) {
        self.queue.insert(item);
    }

    pub fn pop_first_expired(&mut self) -> Option<E> {
        if self.queue.first().map(|e| e.is_expired()).unwrap_or(false) {
            return self.queue.pop();
        }

        None
    }
}

#[cfg(test)]
mod expirable_queue_test {
    use super::*;

    #[derive(Debug, Eq, PartialEq, PartialOrd)]
    struct Expirable {
        ttl: Option<u64>,
    }

    impl Expirable {
        fn new(ttl: Option<u64>) -> Self {
            Self { ttl }
        }
    }

    impl ExpirableTrait<u64> for Expirable {
        fn ttl(&self) -> Option<u64> {
            self.ttl
        }
    }

    impl Ord for Expirable {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.ttl_cmp(other)
        }
    }

    #[test]
    fn test_expirable_queue() {
        let mut queue = ExpirableQueue::new();

        let e1 = Expirable::new(None);
        let e2 = Expirable::new(Some(0));
        let e3 = Expirable::new(Some(1));
        let e4 = Expirable::new(Some(2));

        queue.push(e1);
        queue.push(e2);
        queue.push(e3);
        queue.push(e4);

        println!("{:?}", queue.queue);

        assert_eq!(queue.pop_first_expired(), Some(Expirable::new(Some(0))));
        assert_eq!(queue.pop_first_expired(), None);
        assert_eq!(queue.pop_first_expired(), None);
        assert_eq!(queue.pop_first_expired(), None);
    }
}
