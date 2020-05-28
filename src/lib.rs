use std::collections::VecDeque;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Queue<T: Clone + Default> {
    items: VecDeque<T>,
    capacity: Option<usize>,
}

impl<T: Clone + Default> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: VecDeque::new(), capacity: None }
    }

    pub fn with_cap(limit: usize) -> Queue<T> {
        Queue { items: VecDeque::with_capacity(limit), capacity: Some(limit) }
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

    // pub fn deque(&mut self) -> Result<Option<T>, String> {}

    pub fn capacity(&self) -> usize {
        match self.capacity {
            Some(cap) => cap,
            None => usize::max_value()
        }
    }

    pub fn resize_cap(&mut self, limit: usize) -> Result<Option<T>, String> {
        if self.size() <= limit {
            self.capacity = Some(limit);
            self.items.resize_with(limit, Default::default);
            Ok(None)
        } else {
            Err(format!("Cap cannot be smaller than size, new_cap: {} size: {}",
                        limit,
                        self.size()))
        }
    }

    pub fn remove_cap(&mut self) {
        self.capacity = None;
        self.items.resize_with(usize::max_value(), Default::default);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

impl<T: Clone + Default> Default for Queue<T> {
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
        assert_eq!(Queue::<i32>::new(), Queue { items: VecDeque::new(), capacity: None });
    }

    #[test]
    fn test_default() {
        assert_eq!(Queue::<i32>::default(), Queue { items: VecDeque::new(), capacity: None })
    }

    #[test]
    fn test_new_limited() {
        let queue = Queue::<i32>::with_cap(5);
        assert_eq!(queue, Queue { items: VecDeque::new(), capacity: Some(5) })
    }

    #[test]
    fn test_capacity() {
        let queue = Queue::<i32>::with_cap(5);
        assert_eq!(queue.capacity(), 5);
        let queue2 = Queue::<i32>::new();
        assert_eq!(queue2.capacity(), usize::max_value());
    }

    #[test]
    fn test_add() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.add(3), Ok(None));
    }

    #[test]
    fn test_add_with_capacity() {
        let mut queue: Queue<i32> = Queue::with_cap(3);
        assert_eq!(queue.add(1), Ok(None));
        assert_eq!(queue.add(2), Ok(None));
        assert_eq!(queue.add(3), Ok(None));
        let err = format!("The queue is full, capacity: {} size: {}",
                          queue.capacity(),
                          queue.size());
        assert_eq!(queue.add(4), Err(err));
    }

    #[test]
    fn test_resize() {
        let mut queue: Queue<i32> = Queue::with_cap(3);
        assert_eq!(queue.add(1), Ok(None));
        assert_eq!(queue.add(2), Ok(None));
        assert_eq!(queue.add(3), Ok(None));
        let err = format!("The queue is full, capacity: {} size: {}",
                          queue.capacity(),
                          queue.size());
        assert_eq!(queue.add(4), Err(err));
        queue.resize_cap(6);
        assert_eq!(queue.capacity(), 6);
        assert_eq!(queue.capacity(), queue.size());
        queue.resize_cap(3);
        assert_eq!(queue.capacity(), 6);
        assert_eq!(queue.capacity(), queue.size());
    }

    #[test]
    fn test_size() {}
}
