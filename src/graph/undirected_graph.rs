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

    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    pub fn max_degree(&self) -> usize {
        (0..self.v()).map(|x| self.degree(x)).max().unwrap_or(0)
    }

    pub fn avg_degree(&self) -> f64 {
        2.0 * self.e() as f64 / self.v() as f64
    }

    pub fn number_of_self_loops(&self) -> i32 {
        let mut count = 0;
        for v in 0..self.v() {
            for w in self.adj(v) {
                if v == w {
                    count += 1
                }
            }
        }
        count / 2
    }

    pub fn adj(&self, v: usize) -> std::vec::IntoIter<usize> {
        self.adj[v]
            .iter()
            .map(|v| v.clone())
            .collect::<Vec<usize>>()
            .into_iter()
    }
}

#[test]
fn graph_test() {
    let g = Graph::new(3);
    println!("{:?}", g);
}
