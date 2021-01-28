use std::{collections::HashSet, usize};

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn plus_one(s: String, j: usize) -> String {
        let mut ch: Vec<char> = s.chars().collect();
        if ch[j] == '9' {
            ch[j] = '0';
        } else {
            let digit = ch[j].to_digit(10).unwrap() + 1;
            ch[j] = std::char::from_digit(digit, 10).unwrap();
        }
        return ch.into_iter().collect();
    }

    fn minus_one(s: String, j: usize) -> String {
        let mut ch: Vec<char> = s.chars().collect();
        if ch[j] == '0' {
            ch[j] = '9';
        } else {
            let digit = ch[j].to_digit(10).unwrap() - 1;
            ch[j] = std::char::from_digit(digit, 10).unwrap();
        }
        return ch.into_iter().collect();
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deads: HashSet<String> = deadends.into_iter().collect();
        let mut visited: HashSet<String> = HashSet::new();
        let mut q: Vec<String> = Vec::new();
        let mut step = 0;
        q.push("0000".into());
        visited.insert("0000".into());

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let curr = q.remove(0);
                if deads.contains(&curr) {
                    continue;
                }
                if curr == target {
                    return step;
                }

                for j in 0..4 {
                    let up = Solution::plus_one(curr.clone(), j);
                    if !visited.contains(&up) {
                        q.push(up.clone());
                        visited.insert(up.clone());
                    }
                    let down = Solution::minus_one(curr.clone(), j);
                    if !visited.contains(&down) {
                        q.push(down.clone());
                        visited.insert(down.clone());
                    }
                }
            }
            step += 1;
        }
        return -1;
    }
}
