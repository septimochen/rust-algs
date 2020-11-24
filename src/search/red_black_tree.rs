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