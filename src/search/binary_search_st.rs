#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
struct BinarySearchST {
    keys: Vec<String>,
    vals: Vec<i32>,
    n: usize,
}

#[allow(dead_code)]
impl BinarySearchST {
    pub fn new(capacity: usize) -> Self {
        BinarySearchST {
            keys: Vec::with_capacity(capacity),
            vals: Vec::with_capacity(capacity),
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn rank(&self, key: String) -> usize {
        0
    }

    pub fn get(&self, key: String) -> Option<i32> {
        if self.n == 0 {
            None
        } else {
            let i = self.rank(key.clone());
            if i < self.n && self.keys[i] == key {
                return Some(self.vals[i])
            }
            None
        }
    }
}