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
    pub fn is_red(&self) -> bool {
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
}
