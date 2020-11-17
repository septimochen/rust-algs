pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;

pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where T: Ord;
}