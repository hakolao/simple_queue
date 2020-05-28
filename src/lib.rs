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

    pub fn limit(&self) -> usize {
        match self.limit {
            Some(limit) => limit,
            None => usize::max_value()
        }
    }
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::<T>::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::Queue;

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
}
