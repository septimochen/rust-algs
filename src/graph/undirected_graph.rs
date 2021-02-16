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

#[allow(dead_code)]
pub struct DepthFirstSearch<'a> {
    graph: &'a Graph,
    marked: Vec<bool>,
    count: usize,
}

#[allow(dead_code)]
impl<'a> DepthFirstSearch<'a> {
    pub fn new(g: &'a Graph) -> Self {
        let v = g.v();
        let mut d = DepthFirstSearch {
            graph: g,
            marked: vec![false; v],
            count: 0,
        };
        d.dfs(v);
        d
    }
    pub fn marked(&self, w: usize) -> bool {
        return self.marked[w];
    }

    pub fn count(&self) -> usize {
        self.count
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.count += 1;
        for w in self.graph.adj(v) {
            if !self.marked[w] {
                self.dfs(w);
            }
        }
    }
}

#[test]
fn graph_test() {
    let mut g = Graph::new(3);
    // println!("{:?}", g);
    g.add_edge(1, 2);
    // println!("{:?}", g);
    assert_eq!(g.number_of_self_loops(), 0)
}
