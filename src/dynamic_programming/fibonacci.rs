#[allow(dead_code)]
pub struct Fibonacci;

#[allow(dead_code)]
impl Fibonacci {
    pub fn fib(&self, n: usize) -> usize {
        if n <= 2 {
            return 1;
        } else {
            self.fib(n - 1) + self.fib(n - 2)
        }
    }

    pub fn fib_with_helper(&self, n: usize) -> usize {
        let mut memo = vec![0; n + 1];
        return self.helper(&mut memo, n);
    }

    fn helper(&self, memo: &mut Vec<usize>, n: usize) -> usize {
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
}

#[test]
fn fib_test() {
    let x = Fibonacci.fib_with_helper(6);
    let y = Fibonacci.fib(6);
    assert_eq!(x, 8);
    assert_eq!(y, x);
    let z = Fibonacci.fib(7);
    assert_eq!(z, 13);
}
