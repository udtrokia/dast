extern crate tree;
use tree::node::Node;

type BST<K, V> = Node<K, V>;

fn main() {
    let mut root = BST::<i32, i32>::new(3,4);
    root.insert(2,3);
    root.insert(4,6);
    root.insert(5,5);
    root.insert(6,6);
    root.insert(1,8);
    if let Some(ref left) = root.left {
        assert_eq!(left.value, 3);
    }
}
