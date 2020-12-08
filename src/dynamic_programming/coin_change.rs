use std::cmp::min;
use std::cmp::Ordering;

#[allow(dead_code)]
pub struct CoinChange;

#[allow(dead_code)]
impl CoinChange {
    pub fn coin_change(&self, coins: Vec<i32>, amount: i32) -> i32 {
        fn dp(n: i32, coins: &Vec<i32>, amount: &i32) -> i32 {
            let cmp = n.partial_cmp(&0).unwrap();
            let mut res = amount + 1;
            match cmp {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => {
                    for coin in coins {
                        let sub_problem = dp(n - *coin, coins, &amount);
                        if sub_problem == -1 {
                            continue;
                        }
                        res = min(res, 1 + sub_problem);
                    }
                    res
                }
            }
        }
        dp(amount, &coins, &amount)
    }
}

#[cfg(test)]
mod coin_tests {
    use super::*;
    #[test]
    fn coin_works_1() {
        let x = CoinChange.coin_change(vec![1, 2, 5], 11);
        assert_eq!(x, 3);
    }
}
