#[allow(dead_code)]
pub fn fib(n: usize) -> usize {
    if n <= 2 {
        return 1;
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[test]
fn fib_test() {
    let x = fib(4);
    assert_eq!(x, 3);
}
