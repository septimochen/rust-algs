use std::fmt;

pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct Bag<T> {
    s: Option<Box<Node<T>>>,
    n: usize,
}

