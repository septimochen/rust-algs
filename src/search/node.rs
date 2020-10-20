// use std::cell::RefCell;
// use std::rc::Rc;

// Types
type NodeRef = Box<Node>;
pub type NodeOption = Option<NodeRef>;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub key: String,
    pub val: i32,
    pub next: NodeOption,
}

impl Node {
    pub fn new(text: String) -> Self {
        Node {
            key: text,
            val: 1,
            next: None,
        }
    }
}

impl Drop for Node {
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


#[test]
fn test_new_node() {
    let node = Node::new("node_1".to_string());

    assert_eq!(
        node,
        Node {
            key: "node_1".to_string(),
            val: 1,
            next: None
        }
    );
}
