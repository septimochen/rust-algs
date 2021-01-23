use std::fmt;

#[allow(dead_code)]
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            val: self.val.clone(),
            next: self.next.clone(),
        }
    }
}

#[allow(dead_code)]
fn write_node_to_formatter<T: fmt::Debug>(
    f: &mut fmt::Formatter,
    x: Option<&Box<Node<T>>>,
) -> fmt::Result {
    if let Some(node) = x {
        write!(f, "{:?}", node.val)?;
        write_node_to_formatter(f, node.next.as_ref())
    } else {
        Ok(())
    }
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

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
