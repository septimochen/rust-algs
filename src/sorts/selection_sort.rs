pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    for i in 0..arr.len() {
        let mut s = i;
        for j in i + 1..arr.len() {

            if arr[j] < arr[s] {
                s = j
            }
        }
        arr.swap(i, s)
    }
    arr
}

pub fn run() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    let b = selection_sort(&mut a);
    println!("{:?}", b)
}