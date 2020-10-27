pub mod person;
pub mod binary_search;
pub mod sequential_search_st;
pub mod frequency_counter;
pub mod node;
pub mod binary_search_st;
pub mod bst;

pub trait ST<K, V> {
    fn new() -> Self;

    fn put(&mut self, key: K, val: V);

    fn get(&self, key: &K) -> Option<V>;

    fn delete(&mut self, key: &K);

    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;
}
