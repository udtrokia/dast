// StackNode
#[derive(Debug)]
impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}
