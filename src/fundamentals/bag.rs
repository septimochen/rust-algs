use std::fmt;

#[allow(dead_code)]
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            val: self.val.clone(),
            next: self.next.clone(),
        }
    }
}

#[allow(dead_code)]
fn write_node_to_formatter<T: fmt::Debug>(
    f: &mut fmt::Formatter,
    x: Option<&Box<Node<T>>>,
) -> fmt::Result {
    if let Some(node) = x {
        write!(f, "{:?}, ", node.val)?;
        write_node_to_formatter(f, node.next.as_ref())
    } else {
        Ok(())
    }
}

#[allow(dead_code)]
pub struct Bag<T> {
    s: Option<Box<Node<T>>>,
    n: usize,
}

#[allow(dead_code)]
impl<T> Bag<T> {
    pub fn new() -> Bag<T> {
        Bag { s: None, n: 0 }
    }

    pub fn add(&mut self, val: T) {
        let next = self.s.take();
        self.s = Some(Box::new(Node { val, next }));
        self.n += 1;
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Iter<'a, T>
where
    T: 'a,
{
    node: Option<&'a Box<Node<T>>>,
    nitem: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.nitem == 0 {
            None
        } else {
            let ret = self.node.map(|n| &n.val);
            self.node = self.node.map_or(None, |n| n.next.as_ref());
            self.nitem -= 1;
            ret
        }
    }
}

impl<T: Clone> Clone for Bag<T> {
    fn clone(&self) -> Self {
        Bag {
            s: self.s.clone(),
            n: self.n,
        }
    }
}

#[allow(dead_code)]
impl<T> Bag<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            node: self.s.as_ref(),
            nitem: self.n,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Bag<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        write_node_to_formatter(f, self.s.as_ref())?;
        write!(f, "]")
    }
}

#[cfg(test)]
mod bag_tests {
    use super::*;

    #[test]
    fn bag_test() {
        let mut s = Bag::new();
        assert_eq!(s.len(), 0);
        s.add(1000);
        assert_eq!(s.len(), 1);
        s.add(2000);
        assert_eq!(s.len(), 2);
        println!("{:?}", s);
    }

    #[test]
    fn test_bag_iter() {
        let mut s = Bag::new();
        s.add(100);
        s.add(200);
        s.add(300);

        let mut result = vec![300, 200, 100].into_iter();
        for i in s.iter() {
            assert_eq!(*i, result.next().unwrap());
        }

        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_bag_clone() {
        let mut s = Bag::new();
        s.add(100);
        s.add(200);
        s.add(300);

        let t = s.clone();

        assert_eq!(t.len(), 3);
    }
}
