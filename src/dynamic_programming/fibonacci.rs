#[allow(dead_code)]
pub struct Fibonacci;

#[allow(dead_code)]
impl Fibonacci {
    pub fn fib(n: i32) -> i64 {
        if n <= 0 {
            0
        } else if n <= 2 {
            1
        } else {
            Fibonacci::fib(n - 1) + Fibonacci::fib(n - 2)
        }
    }

    pub fn fib_with_helper(n: i32) -> i64 {
        if n <= 0 {
            0
        } else {
            let mut memo:Vec<i64> = vec![0; (n + 1) as usize];
            Fibonacci::helper(&mut memo, n as usize)
        }
    }

    fn helper(memo: &mut Vec<i64>, n: usize) -> i64 {
        //base case
        if n <= 2 {
            1
        } else {
            match memo[n] {
                0 => {
                    memo[n] = Fibonacci::helper(memo, n - 1) + Fibonacci::helper(memo, n - 2);
                    memo[n]
                }
                _ => memo[n],
            }
        }
    }

    pub fn fib_with_dp(n: i32) -> i64 {
        if n <= 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        } else {
            let mut dp = vec![0; (n + 1) as usize];
            dp[1] = 1;
            dp[2] = 1;
            for i in 3..dp.len() {
                dp[i] = dp[i - 1] + dp[i - 2]
            }
            return dp[n as usize];
        }
    }

    pub fn fib_with_dp_2(n: i32) -> i64 {
        let mut dp = vec![0; (n + 3) as usize];
        dp[1] = 1;
        for i in 0..=n {
            dp[i as usize + 1] += dp[i as usize];
            dp[i as usize + 2] += dp[i as usize];
        }
        return dp[n as usize];
    }

    pub fn fib_final(n: i32) -> i64 {
        if n <= 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        } else {
            let mut prev = 1;
            let mut curr = 1;
            for _ in 3..(n + 1) {
                let sum = prev + curr;
                prev = curr;
                curr = sum;
            }
            curr
        }
    }
}

#[test]
fn fib_test() {
    let x = Fibonacci::fib_with_helper(8);
    let y = Fibonacci::fib(8);
    assert_eq!(x, 21);
    assert_eq!(y, x);
    let z = Fibonacci::fib(7);
    assert_eq!(z, 13);
    let a = Fibonacci::fib_with_dp(50);
    assert_eq!(a, 12586269025);
    let b = Fibonacci::fib_final(50);
    assert_eq!(b, a);
    let c = Fibonacci::fib_with_dp_2(50);
    assert_eq!(c, 12586269025);
}
