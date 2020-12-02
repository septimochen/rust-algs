#[allow(dead_code)]
pub fn binary_search<T: PartialOrd + PartialEq>(item: &T, arr: &[T]) -> i32 {
    // let mut idx_pos = -1;
    let mut lo = 0;
    let mut hi = arr.len() - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if &arr[mid] > item {
            hi = mid - 1;
        } else if &arr[mid] < item {
            lo = mid + 1;
        } else {
            return mid as i32;
        }
    }
    return -1;
}

#[test]
pub fn binary_search_run() {
    let index = binary_search(&"Rust", &vec!["Python", "Php", "Java", "C", "C++", "Rust"]);
    println!("Position: {}", index);

    let index = binary_search(&25, &vec![25, 62, 29, 43, 77]);
    println!("Position: {}", index);

    let index = binary_search(&855, &vec![25, 62, 29, 43, 77]);
    println!("Position: {}", index);
}
