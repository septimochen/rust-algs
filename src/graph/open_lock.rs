use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deads: HashSet<String> = deadends.into_iter().collect();
        let mut visited: HashSet<String> = HashSet::new();
        let mut q:Vec<String> = Vec::new();
        let mut step = 0;
        q.push("0000".into());
        visited.insert("0000".into());
        
        return step
    }
}