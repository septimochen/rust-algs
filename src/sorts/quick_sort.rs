use super::Sorter;

pub struct QuickSort;

fn quick_sort<T: Ord>(slice: &mut [T]) {}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quick_sort(slice)
    }
}
