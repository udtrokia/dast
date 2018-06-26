#[derive(Debug)]
pub struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    pub fn new() -> Self {
        // Self is a `Type`
        Queue{ qdata: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.qdata.push(item);
    }
    
    pub fn pop(&mut self) -> T{
        self.qdata.remove(0)
    }
}
