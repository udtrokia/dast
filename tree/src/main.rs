extern crate tree;
use tree::node::{Node, BinaryTree, BinarySearchTree};

type BST<K, V> = Node<K, V>;

fn main() {
    let mut root = BST::<i32, i32>::new(3,4);
    root.insert(2, 3);
    println!("{:?}", &root);
}
