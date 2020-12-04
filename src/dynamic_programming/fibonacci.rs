#[allow(dead_code)]
pub fn fib(n: usize) -> usize {
    if n <= 2 {
        return 1;
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[allow(dead_code)]
pub fn fib_with_helper(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    return helper(&mut memo, n);
}

#[allow(dead_code)]
fn helper(memo: &mut Vec<usize>, n: usize) -> usize {
    //base case
    if n <= 2 {
        return 1;
    }
    match memo[n] {
        0 => {
            memo[n] = helper(memo, n - 1) + helper(memo, n - 2);
            memo[n]
        }
        _ => memo[n],
    }
}

#[test]
fn fib_test() {
    let x = fib_with_helper(6);
    assert_eq!(x, 8);
}
