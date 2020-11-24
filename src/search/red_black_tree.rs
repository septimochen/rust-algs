use std::iter;
use std::fmt;
use std::mem;
use std::cmp::Ordering;
use super::{ST, OrderedST};
use self::Color::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Black
}

pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub color: Color,
}

