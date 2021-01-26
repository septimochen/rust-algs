use crate::fundamentals::bag::Bag;
use std::iter;

#[derive(Clone, Debug)]
pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Bag<usize>>,
}

#[allow(dead_code)]
impl Graph {
    pub fn new(v: usize) -> Graph {
        Graph {
            v,
            e: 0,
            adj: iter::repeat(Bag::<usize>::new()).take(v).collect(),
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.e += 1;
        self.adj[v].add(w);
        self.adj[w].add(v);
    }
}
