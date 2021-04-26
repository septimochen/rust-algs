pub struct Knapsack;

#[allow(dead_code)]
impl Knapsack {
    pub fn knapsack(w: usize, n: usize, wt: &Vec<usize>, val: &Vec<usize>) -> usize {
        let mut dp = vec![vec![0; w + 1]; n + 1];
        for i in 1..=n {
            for w in 1..=w {
                if w < wt[i - 1] {
                    dp[i][w] = dp[i - 1][w];
                } else {
                    dp[i][w] = std::cmp::max(dp[i - 1][w - wt[i - 1]] + val[i - 1], dp[i - 1][w]);
                }
            }
        }
        dp[n][w]
    }
}

#[test]
fn test() {
    assert_eq!(Knapsack::knapsack(4, 3, &vec![2, 1, 3], &vec![4, 2, 3]), 6);
}
