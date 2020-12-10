use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;

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

    pub fn coin_change_with_memo(&self, coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = HashMap::new();
        fn dp(n: i32, coins: &Vec<i32>, amount: &i32, mut memo: &mut HashMap<i32, i32>) -> i32 {
            if let Some(val) = memo.get(amount) {
                return *val;
            }
            let cmp = n.partial_cmp(&0).unwrap();
            let mut res = amount + 1;
            match cmp {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => {
                    for coin in coins {
                        let sub_problem = dp(n - *coin, coins, &amount, &mut memo);
                        if sub_problem == -1 {
                            continue;
                        }
                        res = min(res, 1 + sub_problem);
                    }
                    if res == amount + 1 {
                        memo.insert(n, -1);
                    } else {
                        memo.insert(n, res);
                    }
                    *memo.get(&n).unwrap()
                }
            }
        }
        dp(amount, &coins, &amount, &mut memo)
    }

    pub fn coin_change_final(&self, coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 {
            return -1;
        }

        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..dp.len() {
            for coin in &coins {
                if i as i32 - *coin < 0 {
                    continue;
                }
                dp[i] = min(dp[i], 1 + dp[i - *coin as usize]);
            }
        }
        let res = dp[amount as usize];
        if res == amount + 1 {
            -1
        } else {
            res
        }
    }
}

#[cfg(test)]
mod coin_tests {
    use super::*;
    #[test]
    fn coin_works_1() {
        let x = CoinChange.coin_change(vec![2], 3);
        assert_eq!(x, -1);
        let y = CoinChange.coin_change(vec![1, 2, 5], 18);
        assert_eq!(y, 5);
    }

    #[test]
    fn coin_works_2() {
        let x = CoinChange.coin_change_with_memo(vec![2], 3);
        assert_eq!(x, -1);
        let y = CoinChange.coin_change_with_memo(vec![2], 1);
        assert_eq!(y, -1);
    }

    #[test]
    fn coin_works_3() {
        let x = CoinChange.coin_change_final(vec![2], 3);
        assert_eq!(x, -1);
        let y = CoinChange.coin_change_final(vec![1, 2, 5], 18);
        assert_eq!(y, 5);
        let z = CoinChange.coin_change_final(vec![1, 2, 5], 100);
        assert_eq!(z, 20);
    }
}
