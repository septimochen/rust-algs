use std::cmp::min;
use std::cmp::Ordering;

#[allow(dead_code)]
pub struct CoinChange {}

#[allow(dead_code)]
impl CoinChange {
    fn coin_change(&self, coins: Vec<i32>, amount: i32) -> i32 {
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
                        res = min(res, sub_problem);
                    }
                    return res;
                }
            }
        }
        dp(amount, &coins, &amount)
    }
}
