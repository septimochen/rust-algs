use std::cell::Cell;
// use std::rc::Rc;

// Types
type ListNodeRef = Box<ListNode>;
pub type ListNodeOption = Option<ListNodeRef>;

type NodeRef = Box<Node>;
pub type NodeOption = Option<NodeRef>;

#[derive(PartialEq, Debug, Clone)]
pub struct ListNode {
    pub key: String,
    pub val: Cell<i32>,
    pub next: ListNodeOption,
}

impl ListNode {
    pub fn new(text: String, value:i32, next:ListNodeOption) -> Self {
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
    pub left: NodeOption,
    pub right: NodeOption,
    pub n: usize,
}

impl Node {
    pub fn new(text: String, value:i32, n:usize) -> Self {
        Node {
            key: text,
            val: value,
            left: None,
            right: None,
            n: n,
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Node with this data -> '{}' just dropped", self.key);
    }
}