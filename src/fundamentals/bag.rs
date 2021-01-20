use std::fmt;

pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}