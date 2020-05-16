pub fn sort<T: Ord>(arr: &mut [T]) -> &[T] {
    let n = arr.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1]{           
            if arr[j] < arr[j-1] {
                arr.swap(j, j-1);
                j -= 1;
            }
        }
    }
    arr
}

pub fn run() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    let b = sort(&mut a);
    println!("{:?}", b)
}