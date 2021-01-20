use std::fmt;

pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct Bag<T> {
    s: Option<Box<Node<T>>>,
    n: usize,
}

impl<T> Bag<T> {
    pub fn new() -> Bag<T> {
        Bag {
            s: None,
            n: 0
        }
    }
}