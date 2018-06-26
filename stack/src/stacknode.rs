// StackNode

#[derive(Debug)]
struct<T> StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}
    
impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}
