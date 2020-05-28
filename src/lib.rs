#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Queue<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: vec![] }
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
        assert_eq!(Queue::<i32>::new(), Queue { items: vec![] });
    }

    #[test]
    fn test_default() {
        assert_eq!(Queue::<i32>::default(), Queue { items: vec![] })
    }
}
