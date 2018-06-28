/// node 
use std::fmt::Display;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;
// type defined.

#[derive(Debug)]
pub struct Node<K, V: Display> {
    pub left: TreeNode<K, V>,
    pub right: TreeNode<K, V>,
    pub key: K,
    pub value: V,
}

