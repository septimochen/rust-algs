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
        return -1;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if &arr[mid] == item {
            right = mid - 1;
        } else if &arr[mid] > item {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    if left >= arr.len() || &arr[left] != item {
        return -1;
    }
    left as i32
}

#[allow(dead_code)]
pub fn right_bound<T: PartialOrd + PartialEq>(item: &T, arr: &[T]) -> i32 {
    let mut left = 0;
    let mut right: i32 = arr.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if &arr[mid as usize] == item {
            left = mid + 1;
        } else if &arr[mid as usize] < item {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    if right < 0 || &arr[right as usize] != item {
        return -1;
    }
    right as i32
}

#[test]
pub fn binary_search_run() {
    let index = binary_search(&"C++", &vec!["C", "C++", "Java", "PHP", "Python", "Rust"]);
    println!("Position: {}", index);

    let index = binary_search(&62, &vec![25, 29, 43, 62, 77]);
    println!("Position: {}", index);

    let index = binary_search(&855, &vec![25, 29, 43, 62, 77]);
    println!("Position: {}", index);
}

#[test]
pub fn left_bound_test() {
    let a = left_bound(&100, &vec![1, 2, 2, 3, 3, 3, 3, 4, 5, 6]);
    assert_eq!(a, -1);
    let b = left_bound(&2, &vec![0, 2, 2, 3, 3, 3, 3, 4, 5, 6]);
    assert_eq!(b, 1);
}

#[test]
pub fn right_bound_test() {
    let a = right_bound(&100, &vec![1, 2, 2, 3, 3, 3, 3, 4, 5, 6]);
    assert_eq!(a, -1);
    let b = right_bound(&2, &vec![0, 2, 2, 3, 3, 3, 3, 4, 5, 6]);
    assert_eq!(b, 2);
}
