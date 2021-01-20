use std::fmt;

#[allow(dead_code)]
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
pub struct Bag<T> {
    s: Option<Box<Node<T>>>,
    n: usize,
}

#[allow(dead_code)]
impl<T> Bag<T> {
    pub fn new() -> Bag<T> {
        Bag { s: None, n: 0 }
    }

    pub fn add(&mut self, val: T) {
        let next = self.s.take();
        self.s = Some(Box::new(Node { val, next }));
        self.n += 1;
    }
}
