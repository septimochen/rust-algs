use std::cmp::{Ordering, Ord};

#[derive(Clone)]
pub enum Node<K:Ord, V> {
    Leaf {
        key: K,
        val: V,
        left: Box<Node<K, V>>,
        right: Box<Node<K, V>>,
        n: i32,
    },
    Empty,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new() -> Self {
        Node::Empty
    }

    pub fn create(key: K, val: V, n:i32) -> Self {
        Node::Leaf {
            key,
            val,
            left: Box::new(Node::Empty),
            right: Box::new(Node::Empty),
            n: n,
        }
    }
}