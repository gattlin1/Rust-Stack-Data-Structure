pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn with_capacity(items: Vec<T>) -> Self {
        Self {
            items,
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut st = Stack::new();
        st.push(21);
        assert_eq!(Some(&21), st.peek());
        assert_eq!(false, st.is_empty());
    }

    #[test]
    fn pop() {
        let mut st = Stack::new();
        st.push("pop test".to_string());
        assert_eq!(Some("pop test".to_string()), st.pop());
    }

    #[test]
    fn peek() {
        let mut st = Stack::new();
        st.push(6.3);
        assert_eq!(Some(&6.3), st.peek());
    }

    #[test]
    fn is_empty() {
        let st: Stack<i32> = Stack::new();
        assert_eq!(true, st.is_empty());
    }

    #[test]
    fn with_capacity() {
        let vector = vec![1, 2, 3, 4, 5];
        let st = Stack::with_capacity(vector);
        assert_eq!(vec![1, 2, 3, 4, 5], st.items);
    }
}