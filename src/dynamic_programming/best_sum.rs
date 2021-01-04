use std::collections::HashMap;
#[allow(dead_code)]
pub struct BestSum;

#[allow(dead_code)]
impl BestSum {
    pub fn best_sum(target_sum: i32, numbers: &Vec<i32>) -> Option<Vec<i32>> {
        let mut memo: HashMap<i32, Option<Vec<i32>>> = HashMap::new();
        BestSum::helper(target_sum, numbers, &mut memo)
    }

    pub fn helper(
        target_sum: i32,
        numbers: &Vec<i32>,
        memo: &mut HashMap<i32, Option<Vec<i32>>>,
    ) -> Option<Vec<i32>> {
        if memo.contains_key(&target_sum) {
            return memo[&target_sum].clone();
        }
        if target_sum == 0 {
            return Some(vec![]);
        }
        if target_sum < 0 {
            return None;
        }

        let mut shortest: Option<Vec<i32>> = None;

        for &num in numbers {
            let remainder = target_sum - num;
            let remainder_result = BestSum::helper(remainder, numbers, memo);
            if remainder_result.is_some() {
                let mut result = vec![num];
                result.append(&mut remainder_result.unwrap());
                let temp = shortest.clone();
                if temp.is_none() || temp.unwrap().len() > result.len() {
                    shortest = Some(result.clone());
                }
            }
        }
        memo.insert(target_sum, shortest.clone());
        return shortest;
    }

    pub fn best_sum_2(target_sum: i32, numbers: &Vec<i32>) -> Option<Vec<i32>> {
        let mut memo: HashMap<i32, Option<Vec<i32>>> = HashMap::new();
        BestSum::helper(target_sum, numbers, &mut memo)
    }
}

#[cfg(test)]
mod best_sum_test {
    use super::*;

    #[test]
    fn best_sum_test_1() {
        let a = BestSum::best_sum(7, &vec![5, 2, 4, 7]);
        println!("{:?}", a);
        let b = BestSum::best_sum(7, &vec![1, 2, 3, 4]);
        println!("{:?}", b);
        let c = BestSum::best_sum(8, &vec![4, 5, 2, 3]);
        println!("{:?}", c);
        let d = BestSum::best_sum(300, &vec![7, 14]);
        assert_eq!(d, None);
    }
}
