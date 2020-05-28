use std::collections::VecDeque;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Queue<T: Clone> {
    items: VecDeque<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: VecDeque::new() }
    }

    pub fn with_cap(cap: usize) -> Queue<T> {
        Queue { items: VecDeque::with_capacity(cap) }
    }

    pub fn add(&mut self, item: T) -> Result<Option<T>, String> {
        if self.size() < self.capacity() {
            self.items.push_back(item);
            Ok(None)
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
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::<T>::new()
    }
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
        assert_eq!(queue.add(3), Ok(None));
    }

    #[test]
    fn test_add_with_capacity() {
        let mut queue: Queue<i32> = Queue::with_cap(3);
        assert!(queue.add(1).is_ok());
        assert!(queue.add(1).is_ok());
        assert!(queue.add(1).is_ok());
        assert!(queue.add(4).is_err());
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
}
