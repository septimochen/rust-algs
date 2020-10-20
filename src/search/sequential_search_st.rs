use super::node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ST {
    fn get(&self, key: String) -> Option<i32> {
        None
    }
    fn put(&mut self, key: String, val: i32) {}
}

#[derive(PartialEq)]
pub struct SequentialSearchST {
    first: NodeOption,
    size: i32,
}

impl SequentialSearchST {
    pub fn new_empty() -> Self {
        SequentialSearchST {
            first: None,
            size: 0,
        }
    }

    pub fn new(key: String) -> Self {
        let new_head = Node::new(key);

        SequentialSearchST {
            first: Some(new_head),
            size: 1,
        }
    }

    fn iter_node(&self) -> ListNodeIterator {
        match &self.first {
            Some(head) => ListNodeIterator::new(Some(Rc::clone(head))),
            _ => ListNodeIterator::new(None),
        }
    }

    pub fn print_items(&self) {
        for node in self.iter_node() {
            println!("the data is {}", node.borrow().key);
        }
    }
}

impl ST for SequentialSearchST {
    fn get(&self, key: String) -> Option<i32> {
        None
    }
    fn put(&mut self, key: String, val: i32) {
        let mut curr = &self.first;
    }
}
