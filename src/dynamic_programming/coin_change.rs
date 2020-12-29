use std::cmp::min;

#[allow(dead_code)]
pub struct CoinChange;

#[allow(dead_code)]
impl CoinChange {
    pub fn coin_change_final(coins: Vec<i32>, amount: i32) -> i32 {
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
    fn coin_works_3() {
        let x = CoinChange::coin_change_final(vec![2], 3);
        assert_eq!(x, -1);
        let y = CoinChange::coin_change_final(vec![1, 2, 5], 18);
        assert_eq!(y, 5);
        let z = CoinChange::coin_change_final(vec![1, 2, 5], 100);
        assert_eq!(z, 20);
    }
}
