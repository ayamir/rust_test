#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone, Debug)]
pub struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    pub fn new(val: T) -> StackNode<T> {
        StackNode { val, next: None }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}
