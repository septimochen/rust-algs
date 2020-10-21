#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
struct BinarySearchST {
    keys: Vec<String>,
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
        // println!("{}", key);
        let mut lo = 0 as usize;
        let mut hi = self.n - 1;
        while lo <= hi {
            let mid = lo + (hi - lo)/2;
            if key < self.keys[mid] {
                hi = mid - 1;
            } else if key > self.keys[mid] {
                lo = mid + 1;
            } else {
                return mid;
            }
        }
        lo
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
        if i < self.n && self.keys[i] == key {
            self.vals[i] = Some(val);
            return;
        }
        let mut j = self.n;
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


#[test]
pub fn binary_search_st_test() {
    let mut st2 = BinarySearchST::new();
    st2.put("ok".to_owned(), 3);
    let v = st2.get(String::from("ok"));
    println!("{:?}", st2);
    assert_eq!(v, Some(3));
}
