#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Queue<T: Clone> {
    items: Vec<T>,
    limit: Option<usize>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: vec![], limit: None }
    }

    pub fn new_limited(limit: Option<usize>) -> Queue<T> {
        Queue { items: vec![], limit }
    }

    pub fn add(&mut self, item: T) -> Result<Option<T>, String> {
        if self.size() < self.limit() {
            self.items.push(item);
            Ok(None)
        } else {
            Err(format!("The queue is full, limit: {} size: {}",
                        self.limit(),
                        self.size()))
        }
    }

    pub fn limit(&self) -> usize {
        match self.limit {
            Some(limit) => limit,
            None => usize::max_value()
        }
    }

    pub fn set_limit(&mut self, limit: usize) -> Result<Option<T>, String> {
        if self.size() <= limit {
            self.limit = Some(limit);
            Ok(None)
        } else {
            Err(format!("Limit cannot be smaller than size, new_limit: {} size: {}",
                        limit,
                        self.size()))
        }
    }

    pub fn remove_limit(&mut self) {
        self.limit = None;
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

    #[test]
    fn test_new() {
        assert_eq!(Queue::<i32>::new(), Queue { items: vec![], limit: None });
    }

    #[test]
    fn test_default() {
        assert_eq!(Queue::<i32>::default(), Queue { items: vec![], limit: None })
    }

    #[test]
    fn test_new_limited() {
        let queue = Queue::<i32>::new_limited(Some(5));
        assert_eq!(queue, Queue { items: vec![], limit: Some(5) })
    }

    #[test]
    fn test_limit() {
        let queue = Queue::<i32>::new_limited(Some(5));
        assert_eq!(queue.limit(), 5);
        let queue2 = Queue::<i32>::new();
        assert_eq!(queue2.limit(), usize::max_value());
    }

    #[test]
    fn test_add() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.add(3), Ok(None));
    }

    #[test]
    fn test_add_with_limit() {
        let mut queue: Queue<i32> = Queue::new_limited(Some(3));
        assert_eq!(queue.add(1), Ok(None));
        assert_eq!(queue.add(2), Ok(None));
        assert_eq!(queue.add(3), Ok(None));
        let err = format!("The queue is full, limit: {} size: {}",
                          queue.limit(),
                          queue.size());
        assert_eq!(queue.add(4), Err(err));
    }

    #[test]
    fn test_size() {}
}
