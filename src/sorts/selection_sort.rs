use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let mut smallest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest] {
                    smallest = i;
                }
            }

            let smallest_v2 = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(x, _)| x + unsorted)
                .expect("the slice is empty");

            assert_eq!(smallest, smallest_v2);
            slice.swap(unsorted, smallest);
        }
    }
}
#[test]
pub fn selection_dumb() {
    let mut a = [1, 2, 3, 4, -1, 3, 2, 1];
    SelectionSort.sort(&mut a);
    assert_eq!(a, [-1, 1, 1, 2, 2, 3, 3, 4]);
}
