use super::node::{Node, Child};
use std::mem::swap;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct BST {
    root: Child,
}

#[allow(dead_code)]
impl BST {
    pub fn new() -> BST{
        BST {
            root: None
        }
    }

    pub fn get(&self, key: String) -> Option<i32> {
        match &self.root {
            None => None,
            Some(ref n) => n.get(&key)
        }
    }
    
    pub fn put(&mut self, key: String, val: i32) {
        match self.root {
            None => {
                swap(&mut self.root, &mut Some(Box::from(Node::new(key, val))));
            },
            Some(ref mut n) => {
                n.put(key, val);
            }
        }
    }
}

#[test]
pub fn bst_test1() {
    let mut bst1 = BST::new();
    bst1.put("ok".to_owned(), 12);
    bst1.put("a".to_string(), 1);
    bst1.put("zed".to_string(), 33);
    println!("{:?}", bst1);
    assert_eq!(bst1.get("zed".to_string()), Some(33))
}
