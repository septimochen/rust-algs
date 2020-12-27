use std::collections::HashMap;
#[allow(dead_code)]
pub struct GridTraveler;

#[allow(dead_code)]
impl GridTraveler {
    pub fn travel(m: usize, n: usize) -> usize {
        let mut memo: HashMap<String, usize> = HashMap::new();
        GridTraveler::helper(m, n, &mut memo)
    }

    pub fn helper(m: usize, n: usize, memo: &mut HashMap<String, usize>) -> usize {
        let key = &[m.to_string(), n.to_string()].join(",");
        if memo.contains_key(&key.clone()) {
            return memo[&key.clone()];
        }
        if m == 1 && n == 1 {
            return 1;
        }
        if m == 0 || n == 0 {
            return 0;
        }
        let value = GridTraveler::helper(m - 1, n, memo) + GridTraveler::helper(m, n - 1, memo);
        memo.insert(key.clone(), value);
        return memo[key];
    }
}

#[cfg(test)]
mod grid_test {
    use super::*;

    #[test]
    pub fn grid_travel_test() {
        let a = GridTraveler::travel(1, 1);
        assert_eq!(a, 1);
        let b = GridTraveler::travel(2, 3);
        assert_eq!(b, 3);
        let c = GridTraveler::travel(3, 2);
        assert_eq!(c, 3);
        let d = GridTraveler::travel(3, 3);
        assert_eq!(d, 6);
        let e = GridTraveler::travel(18, 18);
        assert_eq!(e, 2333606220);
    }
}
