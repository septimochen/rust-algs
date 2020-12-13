use super::Sorter;

#[allow(dead_code)]
pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
    arr
}

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
pub fn bubble_run() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    let b = bubble_sort(&mut a);
    assert_eq!(b, [-1, 1, 1, 2, 2, 3, 3, 4]);
    BubbleSort.sort(&mut a);
    assert_eq!(a, [-1, 1, 1, 2, 2, 3, 3, 4]);
}
