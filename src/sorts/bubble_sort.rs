pub fn bubble_sort<T: Ord>(arr: &mut[T]) -> &[T]{
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1)
            }
        }
    }
    arr
}

#[test]
pub fn bubble_run() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    let b = bubble_sort(&mut a);
    println!("{:?}", b)
}
