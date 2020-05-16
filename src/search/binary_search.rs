// #[allow(unused_assignments)]
pub fn search<T: PartialOrd + PartialEq>(item: &T, arr: &[T]) -> i32 {
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

pub fn run() {
    let index = search(&"Rust", &vec!["Python", "Php", "Java", "C", "C++", "Rust"]);
    println!("Position: {}", index);

    let index = search(&25, &vec![25, 62, 29, 43, 77]);
    println!("Position: {}", index);

    let index = search(&855, &vec![25, 62, 29, 43, 77]);
    println!("Position: {}", index);
}
