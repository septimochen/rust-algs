use self::Color::*;
use super::{OrderedST, ST};
use std::cmp::Ordering;
use std::fmt;
use std::iter;
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
        assert!(self.right.as_ref().map_or(false, |x| x.is_red()));
        let mut x = self.right.take();
        self.right = x.as_mut().unwrap().left.take();
        x.as_mut().unwrap().color = self.color;
        self.color = Red;
        let old_self = mem::replace(self, *x.unwrap());
        self.left = Some(Box::new(old_self));
    }
}
