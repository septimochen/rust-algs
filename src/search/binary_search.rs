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


#[allow(dead_code)]
pub fn left_bound<T: PartialOrd + PartialEq>(item: &T, arr: &[T]) -> i32 {
    if arr.len() == 0 {
        return -1
    }
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = (left + right) / 2;
        if &arr[mid] == item {
            right = mid;
        } else if &arr[mid] > item {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left < arr.len() {
        left as i32
    } else {
        -1
    }
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

#[test]
pub fn left_bound_test() {
    let i = left_bound(&100, &vec![1, 2, 2, 3, 3, 3, 3, 4, 5, 6]);
    assert_eq!(i, -1)
}