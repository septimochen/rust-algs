use std::cell::Cell;
// use std::rc::Rc;
use std::cmp::Ordering;
use std::mem::swap;

// Types
type ListNodeRef = Box<ListNode>;
pub type ListNodeOption = Option<ListNodeRef>;

pub type Child = Option<Box<Node>>;

#[derive(PartialEq, Debug, Clone)]
pub struct ListNode {
    pub key: String,
    pub val: Cell<i32>,
    pub next: ListNodeOption,
}

impl ListNode {
    pub fn new(text: String, value: i32, next: ListNodeOption) -> Self {
        ListNode {
            key: text,
            val: Cell::new(value),
            next: next,
        }
    }
}

impl Drop for ListNode {
    fn drop(&mut self) {
        println!("Node with this data -> '{}' just dropped", self.key);
    }
}

// // Node iterator
// pub struct ListNodeIterator {
//     current: NodeOption,
// }

// impl ListNodeIterator {
//     pub fn new(start_at: NodeOption) -> Self {
//         ListNodeIterator { current: start_at }
//     }
// }

// #[test]
// fn test_new_node() {
//     let node = Node::new("node_1".to_string());

//     assert_eq!(
//         node,
//         Node {
//             key: "node_1".to_string(),
//             val: 1,
//             next: None
//         }
//     );
// }

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    pub key: String,
    pub val: i32,
    pub left: Child,
    pub right: Child,
    pub n: usize,
}

impl Node {
    pub fn new(text: String, value: i32, n: usize) -> Self {
        Node {
            key: text,
            val: value,
            left: None,
            right: None,
            n: n,
        }
    }

    pub fn size(x: Child) -> usize {
        if x == None {
            return 0;
        } else {
            x.unwrap().n
        }
    }

    pub fn _key(&self) -> String {
        self.key.clone()
    }

    pub fn keys(&self) -> Vec<String> {
        let mut ks: Vec<String> = Vec::new();
        ks
    }

    pub fn _value(&self) -> i32 {
        self.val.clone()
    }

    pub fn get(&self, key: &String) -> Option<i32> {
        match key.cmp(&self.key) {
                Ordering::Less => match self.left {
                    None => None,
                    Some(ref n) => Node::get(n, key)
                },
                Ordering::Greater => match self.right {
                    None => None,
                    Some(ref n) => Node::get(n, key)
                },
                Ordering::Equal => Some(self.val),
        }
    }

    pub fn put(&mut self, key: String, val: i32) {
        match key.cmp(&self.key) {
                Ordering::Less => match self.left {
                    None => {
                        swap(&mut self.left, &mut Some(Box::from(Node::new(key, val, 1))))
                    }
                    Some(ref mut n) => {n.put(key, val)},

                },
    
                Ordering::Greater => match self.right {
                    None => {
                        swap(&mut self.right, &mut Some(Box::from(Node::new(key, val, 1))))
                    },
                    Some(ref mut n) => {n.put(key, val)},
                },
                Ordering::Equal => {self.val = val}
        }
        self.n = Node::size(self.left.clone()) + Node::size(self.right.clone()) + 1;
    }
}
