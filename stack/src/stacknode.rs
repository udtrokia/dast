// StackNode

#[derive(Debug)]
pub struct StackNode<T> {
    // <T> means apply to `T` strcut ?
    pub val: T,
    pub next: Option<Box<StackNode<T>>>,
}
    
impl<T> StackNode<T> {
    pub fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}
