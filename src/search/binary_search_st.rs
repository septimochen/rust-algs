#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct BinarySearchST {
    pub keys: Vec<String>,
    vals: Vec<Option<i32>>,
    n: usize,
}

#[allow(dead_code)]
impl BinarySearchST {
    pub fn new() -> Self {
        BinarySearchST {
            keys: Vec::new(),
            vals: Vec::new(),
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn rank(&self, key: String) -> usize {
        if self.n == 0 {
            return 0;
        }

        let mut lo = 0;
        let mut hi: i32 = (self.n - 1) as i32;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            // println!("key: {}, mid: {}", key, mid);
            if key < self.keys[mid as usize] {
                hi = mid - 1;
            } else if key > self.keys[mid as usize] {
                lo = mid + 1;
            } else {
                return mid as usize;
            }
        }
        lo as usize
    }

    pub fn get(&self, key: String) -> Option<i32> {
        if self.n == 0 {
            None
        } else {
            let i = self.rank(key.clone());
            if i < self.n && self.keys[i] == key {
                return self.vals[i];
            }
            None
        }
    }

    pub fn put(&mut self, key: String, val: i32) {
        let i = self.rank(key.clone());
        if i < self.n && self.keys[i] == key.clone() {
            self.vals[i] = Some(val);
            return;
        } else {
            let mut j = self.n;
            self.keys.resize(self.n + 1, String::from(""));
            self.vals.resize(self.n + 1, Some(0));
            while j > i {
                self.keys[j] = self.keys[j - 1].to_string();
                self.vals[j] = self.vals[j - 1];
                j -= 1;
            }
            self.keys[i] = key;
            self.vals[i] = Some(val);
            self.n += 1;
        }
    }
}

#[test]
pub fn binary_search_st_test() {
    let mut st2 = BinarySearchST::new();
    println!("{:?}", st2.keys.capacity());
    st2.put("ok".to_owned(), 3);
    // let v = st2.get(String::from("ok"));
    st2.put("ok".to_owned(), 5);
    st2.put("ok2".to_owned(), 33);
    let v2 = st2.get(String::from("ok2"));
    println!("{:?}", st2.keys.capacity());
    assert_eq!(v2, Some(33));
}
