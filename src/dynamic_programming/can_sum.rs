use std::collections::HashMap;
#[allow(dead_code)]
pub struct CanSum;

#[allow(dead_code)]
impl CanSum {
    pub fn can_sum(target_sum: i32, numbers: &Vec<i32>) -> bool {
        let mut memo: HashMap<i32, bool> = HashMap::new();
        CanSum::helper(target_sum, numbers, &mut memo)
    }

    pub fn helper(target_sum: i32, numbers: &Vec<i32>, memo: &mut HashMap<i32, bool>) -> bool {
        if target_sum == 0 {
            return true;
        }
        if target_sum < 0 {
            return false;
        }
        for &num in numbers {
            if memo.contains_key(&target_sum) {
                return memo[&target_sum];
            }
            let remainder = target_sum - num;
            if CanSum::helper(remainder, numbers, memo) == true {
                memo.insert(target_sum, true);
                return true;
            } else {
                continue;
            }
        }
        memo.insert(target_sum, false);
        return false;
    }
}

#[cfg(test)]
mod can_sum_test {
    use super::*;

    #[test]
    fn can_sum_test_1() {
        let a = CanSum::can_sum(7, &vec![2, 3]);
        assert_eq!(a, true);
        let b = CanSum::can_sum(7, &vec![5, 3, 4, 7]);
        assert_eq!(b, true);
        let c = CanSum::can_sum(7, &vec![2, 4]);
        assert_eq!(c, false);
        let d = CanSum::can_sum(8, &vec![2, 3, 5]);
        assert_eq!(d, true);
        let e = CanSum::can_sum(300, &vec![7, 14]);
        assert_eq!(e, false);
    }
}
