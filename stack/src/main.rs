extern crate stack;

use stack::stack::Stack;

fn main() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct { a: i32 };
    // TestStruct defined here, but uses struct `StackNode`
    
    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };
    
    let mut s = Stack::<&TestStruct>::new();
    // Usage: Stack<T>  || => Stack::<&TestStruct>
    // TestStruct inherit StackStruct.
    
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("\n{:?}\n", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
