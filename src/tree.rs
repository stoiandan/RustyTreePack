use std::{borrow::BorrowMut, rc::{Rc, Weak}};


pub struct Tree<T> where T : PartialOrd {
    pub root : NodeRef<T>
}



type NodeRef<T> = Option<Rc<Node<T>>>;
type WeakNodeRef<T> = Weak<Node<T>>;

#[derive(Debug)]
pub struct Node<T> {
    pub left   : NodeRef<T>,
    pub right  : NodeRef<T>,
    pub parent : WeakNodeRef<T>,
    pub value  : T
}

impl<T> Node<T> {
    pub fn new(value : T) -> Self {
        Node { left: None,
               right: None,
               parent: Weak::new(),
               value }
    }
}

impl<T> Tree<T> where T : PartialOrd {

    pub fn new(root_value: T) -> Self {
        Tree {
            root : Some(Rc::new(Node::new(root_value)))
        }
    }

    pub fn insert(&mut self, node_val : T) -> &Self {
        if self.root.is_none() {
            self.root = Some(Rc::new(Node::new(node_val)));
            return self
        }

        let root =   self.root.as_mut().unwrap();
        if  node_val >= root.value  {
            self.right_insert(root.borrow_mut(), node_val); 
        } else {
            self.left_insert(root.borrow_mut(), node_val);
        }

        self
    }

    pub fn left_insert(&mut self, root: &mut Node<T>, value : T) {
        if let Some(_) = &root.left {
            self.insert( value);
        } else {
            root.left = Some(Rc::new(Node::new(value)));
        }
    }

    pub fn right_insert(&mut self, root: &mut  Node<T>, value : T) {
        if let Some(_) = root.right {
            self.insert( value);
        } else {
            root.right = Some(Rc::new(Node::new(value)));
        }
    }
}


#[cfg(test)]
mod tree_test {
    use super::{Tree};
    #[test]
    fn no_empty_root() {
        let tree = Tree::new(5);
        assert_eq!(tree.root.is_none(),false);
    }
}