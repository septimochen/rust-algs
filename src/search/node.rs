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
#[allow(dead_code)]
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
        let mut _ks: Vec<String> = Vec::new();
        _ks
    }

    pub fn _value(&self) -> i32 {
        self.val.clone()
    }

    pub fn get(&self, key: &String) -> Option<i32> {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                None => None,
                Some(ref n) => Node::get(n, key),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref n) => Node::get(n, key),
            },
            Ordering::Equal => Some(self.val),
        }
    }

    pub fn put(&mut self, key: String, val: i32) {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                None => swap(&mut self.left, &mut Some(Box::from(Node::new(key, val, 1)))),
                Some(ref mut n) => n.put(key, val),
            },

            Ordering::Greater => match self.right {
                None => swap(
                    &mut self.right,
                    &mut Some(Box::from(Node::new(key, val, 1))),
                ),
                Some(ref mut n) => n.put(key, val),
            },
            Ordering::Equal => self.val = val,
        }
        self.n = Node::size(self.left.clone()) + Node::size(self.right.clone()) + 1;
    }

    pub fn min(&self) -> Node {
        let node = self.clone();
        if node.left == None {
            self.clone()
        } else {
            node.left.unwrap().min()
        }
    }

    pub fn max(&self) -> Node {
        let node = self.clone();
        if node.right == None {
            self.clone()
        } else {
            node.right.unwrap().max()
        }
    }

    pub fn floor(x: &Child, key: String) -> Child {
        if *x == None {
            return None;
        } else {
            match key.cmp(&x.clone().unwrap().key) {
                Ordering::Equal => x.clone(),
                Ordering::Less => {
                    if x.as_ref().unwrap().left == None {
                        return None;
                    } else {
                        Node::floor(&x.as_ref().unwrap().left, key)
                    }
                }
                Ordering::Greater => {
                    let node = x.clone().unwrap().right;
                    let t = Node::floor(&node, key);
                    if t == None {
                        return x.clone();
                    } else {
                        return t;
                    }
                }
            }
        }
    }

    pub fn select(x: &Child, k: usize) -> Child {
        if k >= Node::size(x.clone()) {
            panic!("out of bounds")
        }
        let t = Node::size(x.clone().unwrap().left);
        if k == t {
            return x.clone();
        } else if k < t {
            Node::select(&x.clone().unwrap().left, k)
        } else {
            Node::select(&x.clone().unwrap().right, k - t - 1)
        }
    }

    pub fn rank(&self, key: &String) -> i32 {
        if self.get(key) == None {
            return -1;
        }
        match key.cmp(&self.key) {
            Ordering::Equal => Node::size(self.left.clone()) as i32,
            Ordering::Less => self.left.clone().unwrap().rank(key),
            Ordering::Greater => {
                self.right.clone().unwrap().rank(key) + Node::size(self.left.clone()) as i32 + 1
            }
        }
    }

    pub fn delete_min(&mut self) -> Child {
        match self.left.clone() {
            None => self.right.clone(),
            Some(_) => {
                self.left = self.left.clone().unwrap().delete_min();
                self.n = Node::size(self.left.clone()) + Node::size(self.right.clone()) + 1;
                return Some(Box::from(self.clone()));
            },
        }
    }

    pub fn print(x: Child) {
        if x == None {
            return
        } else {
            Node::print(x.clone().unwrap().left);
            print!(" {}", x.clone().unwrap().key);
            Node::print(x.clone().unwrap().right);
        }
    }
}
