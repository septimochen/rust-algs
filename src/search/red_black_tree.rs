use self::Color::*;
// use super::{OrderedST, ST};
use std::cmp::Ordering;
// use std::fmt;
// use std::iter;
use std::mem;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Black,
}

pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub color: Color,
}

#[allow(dead_code)]
impl<K, V> Node<K, V> {
    #[inline]
    pub fn new(key: K, val: V, color: Color) -> Node<K, V> {
        Node {
            key: key,
            val: val,
            left: None,
            right: None,
            color: color,
        }
    }

    #[inline]
    fn is_red(&self) -> bool {
        self.color == Red
    }

    fn depth(&self) -> usize {
        let lsz = self.left.as_ref().map_or(0, |n| n.depth());
        let rsz = self.right.as_ref().map_or(0, |n| n.depth());
        if rsz > lsz {
            rsz + 1
        } else {
            lsz + 1
        }
    }

    fn size(&self) -> usize {
        1 + self.left.as_ref().map_or(0, |n| n.size()) + self.right.as_ref().map_or(0, |n| n.size())
    }

    /// Left rotation. Orient a (temporarily) right-leaning red link to lean left.
    fn rotate_left(&mut self) {
        // assert!(self.right.as_ref().map_or(false, |x| x.is_red()));
        assert!(is_red(&self.right));
        let mut x = self.right.take();
        self.right = x.as_mut().unwrap().left.take();
        x.as_mut().unwrap().color = self.color;
        self.color = Red;
        let old_self = mem::replace(self, *x.unwrap());
        self.left = Some(Box::new(old_self));
    }

    /// Right rotation. Orient a left-leaning red link to (temporarily) lean right
    fn rotate_right(&mut self) {
        // assert!(self.left.as_ref().map_or(false, |x| x.is_red()));
        assert!(is_red(&self.left));
        let mut x = self.left.take();
        self.left = x.as_mut().unwrap().right.take();
        x.as_mut().unwrap().color = self.color;
        self.color = Red;
        let old_self = mem::replace(self, *x.unwrap());
        self.right = Some(Box::new(old_self));
    }

    fn flip_color(&mut self) {
        assert!(!self.is_red());
        // assert!(Node::is_red(&self.left.as_ref().unwrap()));
        // assert!(self.right.as_ref().unwrap().is_red());
        assert!(is_red(&self.left));
        assert!(is_red(&self.right));
        self.color = Red;
        self.left.as_mut().map(|n| n.color = Black);
        self.right.as_mut().map(|n| n.color = Black);
    }
}

fn is_red<K, V>(node: &Option<Box<Node<K, V>>>) -> bool {
    if node.as_ref().is_none() {
        false
    } else {
        node.as_ref().unwrap().color == Red
    }
}

fn _put<K: PartialOrd, V>(x: Option<Box<Node<K, V>>>, key: K, val: V) -> Option<Box<Node<K, V>>> {
    let mut x = x;
    if x.is_none() {
        return Some(Box::new(Node::new(key, val, Red)));
    }
    let cmp = key.partial_cmp(&x.as_ref().unwrap().key).unwrap();
    match cmp {
        Ordering::Less => {
            let left = x.as_mut().unwrap().left.take();
            x.as_mut().unwrap().left = _put(left, key, val);
        }
        Ordering::Greater => {
            let right = x.as_mut().unwrap().right.take();
            x.as_mut().unwrap().right = _put(right, key, val);
        }
        Ordering::Equal => {
            x.as_mut().unwrap().val = val;
        }
    }

    if is_red(&x.as_ref().unwrap().right) && !is_red(&x.as_ref().unwrap().left) {
        x.as_mut().unwrap().rotate_left();
    }

    if is_red(&x.as_ref().unwrap().left) && is_red(&x.as_ref().unwrap().left.as_ref().unwrap().left)
    {
        x.as_mut().unwrap().rotate_right();
    }

    if is_red(&x.as_ref().unwrap().left) && is_red(&x.as_ref().unwrap().right) {
        x.as_mut().unwrap().flip_color();
    }

    x
}
