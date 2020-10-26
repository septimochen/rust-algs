use super::node::{Child, Node};
use std::mem::swap;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct BST {
    pub root: Child,
}

#[allow(dead_code)]
impl BST {
    pub fn new() -> BST {
        BST { root: None }
    }

    pub fn size(&self) -> usize {
        Node::size(self.root.clone())
    }

    pub fn min(&self) -> String {
        self.root.as_ref().unwrap().min().key
    }

    pub fn max(&self) -> String {
        self.root.as_ref().unwrap().max().key
    }

    pub fn get(&self, key: String) -> Option<i32> {
        match &self.root {
            None => None,
            Some(ref n) => n.get(&key),
        }
    }

    pub fn keys<'a>(&'a self) -> ::std::vec::IntoIter<&'a String> {
        let mut queue: Vec<&'a String> = Vec::new();
        fn inorder<'a>(x: Option<&'a Box<Node>>, queue: &mut Vec<&'a String>) {
            if x.is_none() {
                return;
            }
            inorder(x.unwrap().left.as_ref(), queue);
            queue.push(&x.unwrap().key);
            inorder(x.unwrap().right.as_ref(), queue);
        };
        inorder(self.root.as_ref(), &mut queue);
        queue.into_iter()
    }

    pub fn put(&mut self, key: String, val: i32) {
        match self.root {
            None => {
                swap(&mut self.root, &mut Some(Box::from(Node::new(key, val, 1))));
            }
            Some(ref mut n) => {
                n.put(key, val);
            }
        }
    }

    pub fn floor(&self, key: String) -> Option<String> {
        let node = Node::floor(&self.root, key);
        match node {
            None => None,
            Some(n) => Some(n.key),
        }
    }

    pub fn select(&self, k: usize) -> Option<String> {
        let node = Node::select(&self.root, k);
        match node {
            None => None,
            Some(n) => Some(n.key),
        }
    }

    pub fn rank(&self, key: &String) -> i32 {
        self.root.as_ref().unwrap().rank(key)
    }

    pub fn delete_min(&mut self) {
        self.root = Node::delete_min(self.root.take()).0;
    }

    pub fn delete_max(&mut self) {
        self.root = Node::delete_max(self.root.take()).0;
    }

    pub fn print(&self) {
        Node::print(self.root.clone())
    }

    pub fn delete(&mut self, key: &String) {
        self.root = Node::delete(self.root.take(), key);
    }
}

#[test]
pub fn bst_test1() {
    let mut bst1 = BST::new();
    bst1.put("ok".to_owned(), 12);
    bst1.put("a".to_string(), 1);
    bst1.put("zed".to_string(), 33);
    // println!("{:?}", bst1.floor("A".to_owned()));
    // println!("{:?}", bst1.rank(&"zed1".to_owned()));
    println!("{:?}", bst1.print());
    // bst1.delete_min();
    // println!("{:?}", bst1.print());
    bst1.delete(&"a".to_owned());
    println!("{:?}", bst1.print());
    assert_eq!(bst1.get("zed".to_string()), Some(33));
    bst1.delete_min();
    println!("{:?}", bst1.print());
}
