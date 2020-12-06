use super::Sorter;

pub struct QuickSort;

fn quick_sort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return
        },
        _ => {}
    }

    
}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quick_sort(slice)
    }
}
