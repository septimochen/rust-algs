#[allow(dead_code)]
pub struct Fibonacci;

#[allow(dead_code)]
impl Fibonacci {
    pub fn fib(&self, n: i32) -> i32 {
        if n <= 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        } else {
            self.fib(n - 1) + self.fib(n - 2)
        }
    }

    pub fn fib_with_helper(&self, n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let mut memo = vec![0; (n + 1) as usize];
        return self.helper(&mut memo, n as usize);
    }

    fn helper(&self, memo: &mut Vec<i32>, n: usize) -> i32 {
        //base case
        if n <= 2 {
            return 1;
        }
        match memo[n] {
            0 => {
                memo[n] = self.helper(memo, n - 1) + self.helper(memo, n - 2);
                memo[n]
            }
            _ => memo[n],
        }
    }

    pub fn fib_with_dp(&self, n: i32) -> i32 {
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
}

#[test]
fn fib_test() {
    let x = Fibonacci.fib_with_helper(6);
    let y = Fibonacci.fib(6);
    assert_eq!(x, 8);
    assert_eq!(y, x as i32);
    let z = Fibonacci.fib(7);
    assert_eq!(z, 13);
    let a = Fibonacci.fib_with_dp(7);
    assert_eq!(a, z);
}
