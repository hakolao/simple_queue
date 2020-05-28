use std::collections::VecDeque;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Queue<T: Clone> {
    items: VecDeque<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: VecDeque::new() }
    }

    pub fn from_vec(vec: Vec<T>) -> Queue<T> {
        Queue { items: VecDeque::from(vec) }
    }

    pub fn items(&self) -> &VecDeque<T> {
        &self.items
    }

    pub fn with_cap(cap: usize) -> Queue<T> {
        Queue { items: VecDeque::with_capacity(cap) }
    }

    pub fn add(&mut self, item: T) -> Result<Option<&T>, String> {
        if self.size() < self.capacity() {
            self.items.push_back(item);
            Ok(self.items.back())
        } else {
            Err(format!("The queue is full, capacity: {} size: {}",
                        self.capacity(),
                        self.size()))
        }
    }

    pub fn deque(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::<T>::new()
    }
}

#[macro_export]
macro_rules! queue {
    () => { Queue::new() };
    ($($x:expr),+) => {
        {
            let mut queue = Queue::default();
            $(queue.add($x).expect("");)*
            queue
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{Queue};
    use std::collections::VecDeque;

    #[test]
    fn test_new() {
        assert_eq!(Queue::<i32>::new(), Queue { items: VecDeque::new() });
    }

    #[test]
    fn test_default() {
        assert_eq!(Queue::<i32>::default(), Queue { items: VecDeque::new() })
    }

    #[test]
    fn test_new_limited() {
        let queue = Queue::<i32>::with_cap(5);
        assert_eq!(queue, Queue { items: VecDeque::new() })
    }

    #[test]
    fn test_add() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.add(3), Ok(Some(&3)));
        assert_eq!(queue.add(5), Ok(Some(&5)));
    }

    #[test]
    fn test_add_with_capacity() {
        let mut queue: Queue<i32> = Queue::with_cap(3);
        assert!(queue.add(1).is_ok());
        assert!(queue.add(1).is_ok());
        assert!(queue.add(3).is_ok());
        assert!(queue.add(4).is_err());
        assert_eq!(queue.get(2), Some(&3));
    }

    #[test]
    fn test_size() {
        let mut queue: Queue<i32> = Queue::with_cap(3);
        queue.add(1).expect("Adding to queue");
        assert_eq!(queue.size(), 1);
        queue.add(1).expect("Adding to queue");
        assert_eq!(queue.size(), 2);
        queue.add(1).expect("Adding to queue");
        assert_eq!(queue.size(), 3);
        assert!(queue.add(1).is_err());
        assert_eq!(queue.capacity(), queue.size());
    }

    #[test]
    fn test_deque() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.size(), 0);
        queue.add(5).expect("Adding to queue");
        queue.add(45).expect("Adding to queue");
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.deque(), Some(5));
        assert_eq!(queue.deque(), Some(45));
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_macro() {
        let mut queue = queue![];
        queue.add(1).expect("Adding to queue");
        queue.add(2).expect("Adding to queue");
        assert_eq!(queue.get(0), Some(&1));
        assert_eq!(queue.get(1), Some(&2));
        let queue2 = queue![1, 2, 3, 4];
        assert_eq!(queue2.get(3), Some(&4));
    }

    #[test]
    fn test_from_vec() {
        let queue = Queue::from_vec(vec![1, 2, 3]);
        assert_eq!(queue.items, VecDeque::from(vec![1, 2, 3]));
    }
}
