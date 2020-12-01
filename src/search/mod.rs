pub mod binary_search;
pub mod binary_search_st;
pub mod bst;
pub mod frequency_counter;
pub mod node;
pub mod person;
pub mod red_black_tree;
pub mod sequential_search_st;

pub trait ST<K, V> {
    fn new() -> Self;

    fn put(&mut self, key: K, val: V);

    fn get(&self, key: &K) -> Option<&V>;

    fn delete(&mut self, key: &K);

    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;
}

pub trait OrderedST<K, V> {
    fn min(&self) -> Option<&K>;

    fn max(&self) -> Option<&K>;

    fn floor(&self, key: &K) -> Option<&K>;

    fn ceiling(&self, key: &K) -> Option<&K>;

    fn rank(&self, key: &K) -> usize;

    fn select(&self, k: usize) -> Option<&K>;

    fn delete_min(&mut self);

    fn delete_max(&mut self);

    fn size_of_key_range(&self, lo: &K, hi: &K) -> usize {
        self.rank(hi) - self.rank(lo)
    }
}
