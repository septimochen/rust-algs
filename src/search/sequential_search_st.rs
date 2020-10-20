use super::node::*;
use std::cell::RefCell;
// use std::rc::Rc;
// use std::borrow::Borrow;

pub trait ST {
    fn get(&self, key: String) -> Option<i32> {
        println!("{}", key);
        None
    }
    fn put(&mut self, key: String, val: i32) {
        println!("{}, {}", key, val);
    }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub struct SequentialSearchST {
    first: NodeOption,
    size: i32,
}

#[allow(dead_code)]
impl SequentialSearchST {
    pub fn new_empty() -> Self {
        SequentialSearchST {
            first: None,
            size: 0,
        }
    }

    pub fn new(key: String, val: i32, next: NodeOption) -> Self {
        let new_head = Node::new(key, val, next);

        SequentialSearchST {
            first: Some(Box::new(new_head)),
            size: 1,
        }
    }

}

impl ST for SequentialSearchST {
    fn get(&self, key: String) -> Option<i32> {
        let curr = self.first.as_ref();
        while curr.is_some() {
            if curr.as_ref().unwrap().key == key {
                return Some(curr.as_ref().unwrap().val)
            }
        }
        None
    }
    fn put(&mut self, key: String, val: i32) {
        let mut curr = self.first.as_mut();
        let new_key = RefCell::new(key);
        while curr.is_some() {
            if curr.as_ref().unwrap().key == new_key.borrow().to_string() {
                println!("{:?}", curr);
                curr.as_mut().unwrap().val = val;
                return;
            }
            curr = curr.unwrap().next.as_mut();
        }
        self.first = Some(Box::new(Node::new(new_key.borrow().to_string(), val, self.first.clone())));
    }
}

#[test]
fn st_test() {
    let mut st1 = SequentialSearchST::new(String::from("ok"), 1, None);
    st1.put("ok".to_string(), 3);
    println!("{:?}", st1.get(String::from("ok")));
    st1.put("ok1".to_owned(), 34);
    println!("{:?}", st1);
    assert_eq!(st1.get("ok1".to_owned()).unwrap(), 34);
}
