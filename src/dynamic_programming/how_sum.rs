use std::collections::HashMap;
#[allow(dead_code)]
pub struct HowSum;

#[allow(dead_code)]
impl HowSum {
    pub fn how_sum(target_sum: i32, numbers: &Vec<i32>) -> Option<Vec<i32>> {
        let mut memo: HashMap<i32, Option<Vec<i32>>> = HashMap::new();
        HowSum::helper(target_sum, numbers, &mut memo)
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
        for &num in numbers {
            let remainder = target_sum - num;
            let remainder_result = HowSum::helper(remainder, numbers, memo);
            if remainder_result.is_some() {
                let mut result = vec![num];
                result.append(&mut remainder_result.unwrap());
                memo.insert(target_sum, Some(result.clone()));
                return Some(result.clone());
            }
        }
        memo.insert(target_sum, None);
        return None;
    }
    pub fn how_sum_2(target_sum: i32, numbers: &Vec<i32>) -> Option<Vec<i32>> {
        let key = target_sum as usize;
        let mut table: Vec<Option<Vec<i32>>> = vec![None; (target_sum + 1) as usize];
        table[0] = Some(vec![]);
        for i in 0..=key {
            if table[i].is_some() {
                for num in numbers {
                    let next = (i as i32 + num) as usize;
                    if next <= key {
                        match table[next].clone() {
                            None => {
                                table[next] = Some(vec![]);
                                table[next].as_mut().unwrap().push(*num);
                            }
                            Some(mut val) => {
                                val.push(*num);
                                table[next] = Some(val);
                            }
                        }
                    }
                }
            }
        }
        table[key].clone()
    }
}

#[cfg(test)]
mod how_sum_test {
    use super::*;

    #[test]
    fn how_sum_test_1() {
        let a = HowSum::how_sum(7, &vec![5, 3, 4, 7]);
        println!("{:?}", a);
        let b = HowSum::how_sum(7, &vec![2, 3]);
        println!("{:?}", b);
        let c = HowSum::how_sum(8, &vec![5, 2, 3]);
        println!("{:?}", c);
        let d = HowSum::how_sum(300, &vec![7, 14]);
        assert_eq!(d, None);
    }
    #[test]
    fn how_sum_test_2() {
        let a = HowSum::how_sum_2(7, &vec![5, 3, 4, 7]);
        println!("{:?}", a);
        let b = HowSum::how_sum_2(7, &vec![2, 3]);
        println!("{:?}", b);
        let c = HowSum::how_sum_2(8, &vec![5, 2, 3]);
        println!("{:?}", c);
        let d = HowSum::how_sum_2(300, &vec![7, 14]);
        assert_eq!(d, None);
    }
}
