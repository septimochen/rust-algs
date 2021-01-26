use crate::fundamentals::bag::Bag;
use std::iter;

#[derive(Clone, Debug)]
pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Bag<usize>>,
}

impl Graph {
    pub fn new(v: usize) -> Graph {
        Graph {
            v: v,
            e: 0,
            adj: iter::repeat(Bag::<usize>::new()).take(v).collect(),
        }
    }
}
