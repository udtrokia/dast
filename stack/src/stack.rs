// Stack
use stacknode::StackNode;
#[derive(Debug)]
pub struct Stack<T> {
    pub top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack{ top: None }
    }

    pub fn push(&mut self, val: T) {
        // the trait `std::fmt::Debug` is not implemented for `T`
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop (&mut self) -> Option<T> {
        let val = self.top.take();
        println!("{:?}", &val.is_none());
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}
