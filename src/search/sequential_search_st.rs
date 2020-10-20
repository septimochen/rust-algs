use super::node::Node;

pub trait ST {
    fn get(&self, key: String) -> Option<i32> {
        None
    }
    fn put(&mut self, key:String, val:i32) {}

}

#[derive(PartialEq)]
pub struct SequentialSearchST
{
    first: Node,
    size: i32,
}

impl ST for SequentialSearchST {
    fn get(&self, key: String) -> Option<i32> {
        None
    }
    fn put(&mut self, key:String, val:i32) {
        let mut x = &self.first;
    }
}