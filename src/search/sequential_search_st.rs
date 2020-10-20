#[allow(dead_code)]
pub struct Node<K, V> {
    key: K,
    val: V,
    next: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Node<K, V> {
        Node{key: key, val: val, next:None}
    }
}

pub struct SequentialSearchST<K, V> {
    first: Node<K, V>,
}