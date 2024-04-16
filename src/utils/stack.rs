pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
        assert!(stack.is_empty());
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
    }
}