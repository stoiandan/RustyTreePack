use std::fmt::{Debug, Display};
use std::ops::Index;
use std::result;

#[derive(Debug, Clone)]
struct NoNodeErr;

type Result<T> = result::Result<Option<T>, NoNodeErr>;

pub struct Tree<T>
where
    T: PartialOrd,
    T: Debug,
    T: Display,
{
    nodes: Vec<Option<T>>,
}

impl<T> Tree<T>
where
    T: PartialOrd,
    T: Debug,
    T: Display,
{
    pub fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    pub fn insert_into(&mut self, val: T) {
        self.insert(0, val);
    }

    pub fn show_nodes(&self) {
        for node in self.nodes.iter() {
            match node {
                Some(t) => println!("{} ", t),
                None => println!("Empty cell "),
            }
        }
    }

    fn insert(&mut self, idx: usize, val: T) {
        match self.get_at(idx) {
            Ok(None) => self.nodes[idx] = Some(val),
            Ok(Some(t)) => {
                // insert to the left
                if t >= &val {
                    self.insert(((idx + 1) * 2) - 1, val)
                } else {
                    // insert to the right
                    self.insert((idx + 1) * 2, val)
                }
            }
            _ => {
                if idx == 0 {
                    self.nodes.push(Some(val));
                    return;
                }
                if idx % 2 == 1 {
                    self.insert_left(val);
                } else {
                    self.insert_right(val);
                }
            }
        };
    }

    fn insert_left(&mut self, val: T) {
        self.nodes.push(Some(val));
        self.nodes.push(None);
    }

    fn insert_right(&mut self, val: T) {
        self.nodes.push(None);
        self.nodes.push(Some(val));
    }

    fn get_left(&self, idx: usize) -> Result<&T> {
        self.get_at(idx * 2)
    }

    fn get_right(&self, idx: usize) -> Result<&T> {
        self.get_at((idx * 2) + 1)
    }

    fn get_at(&self, idx: usize) -> Result<&T> {
        if idx >= self.nodes.len() {
            Err(NoNodeErr)
        } else {
            Ok(self.nodes[idx].as_ref())
        }
    }

    pub fn get_elem(&self, key: T) -> Option<&T> {
        return match self.find(0, &key) {
            Some(t) => self.nodes[t].as_ref(),
            None => None,
        };
    }

    fn find(&self, idx: usize, key: &T) -> Option<usize> {
        match self.get_at(idx) {
            Ok(Some(t)) => {
                if t == key {
                    return Some(idx);
                }
                
                if t >= key {
                    return self.find(((idx + 1) * 2) - 1, key);
                } else {
                    return self.find((idx + 1) * 2, key);
                }
            }
            _ => None,
        }
    }

    fn root(&self) -> Result<&T> {
        self.get_at(0)
    }
}

impl<T> Index<usize> for Tree<T>
where
    T: PartialOrd,
    T: Debug,
    T: Display,
{
    type Output = T;

    fn index<'a>(&'a self, i: usize) -> &T {
        self.nodes[i].as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    #[test]
    fn insert_root() {
        let mut tree = Tree::new();

        tree.insert_into(5);

        assert_eq!(tree[0], 5)
    }

    #[test]
    fn insert_left() {
        let mut tree = Tree::new();

        tree.insert_into(5);
        tree.insert_into(4);

        assert_eq!(tree.get_elem(4).unwrap(), &4);
        tree.show_nodes();
    }

    #[test]
    fn insert_right() {
        let mut tree = Tree::new();

        tree.insert_into(5);
        tree.insert_into(7);

        assert_eq!(tree.get_elem(7).unwrap(), &7);
        tree.show_nodes();
    }

    #[test]
    fn insert_right_and_left() {
        let mut tree = Tree::new();

        tree.insert_into(5);
        tree.insert_into(7);
        tree.insert_into(3);

        assert_eq!(tree.get_elem(3).unwrap(), &3);
        tree.show_nodes();
    }
}
