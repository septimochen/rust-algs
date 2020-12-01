use super::Sorter;

#[allow(dead_code)]
pub fn insertion_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    let n = arr.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
    arr
}

pub struct InsertionSort {
    smart: bool,
}

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let smart = self.smart;
        {
            for unsorted in 1..slice.len() {
                if !smart {
                    let mut i = unsorted;
                    while i > 0 && slice[i - 1] > slice[i] {
                        slice.swap(i - 1, i);
                        i -= 1;
                    }
                } else {
                    let i = slice[..unsorted]
                        .binary_search(&slice[unsorted])
                        .unwrap_or_else(|x| x);
                    &slice[i..=unsorted].rotate_right(1);
                }
            }
        }
    }
}
#[test]
pub fn insertion_dumb() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    let b = insertion_sort(&mut a);
    println!("{:?}", b);
    InsertionSort { smart: false }.sort(&mut a);
    assert_eq!(a, [-1, 1, 1, 2, 2, 3, 3, 4]);
}

#[test]
pub fn insertion_smart() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    let b = insertion_sort(&mut a);
    println!("{:?}", b);
    InsertionSort { smart: true }.sort(&mut a);
    assert_eq!(a, [-1, 1, 1, 2, 2, 3, 3, 4]);
}
