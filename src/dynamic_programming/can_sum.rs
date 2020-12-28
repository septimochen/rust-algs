pub struct CanSum;

impl CanSum {
    pub fn can_sum(target_sum: i32, numbers: &Vec<i32>) -> bool {
        if target_sum == 0 {
            return true;
        } else if target_sum < 0 {
            return false;
        } else {
            for num in numbers {
                let remainder = target_sum - *num;
                if CanSum::can_sum(remainder, numbers) == true {
                    return true;
                }
            }
            return false;
        }
    }
}

#[cfg(test)]
mod can_sum_test {
    use super::*;

    #[test]
    fn can_sum_test_1() {
        let a = CanSum::can_sum(7, &vec![2, 3]);
        assert!(a, true);
    }
}
