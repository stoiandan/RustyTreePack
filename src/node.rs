use std::{borrow::BorrowMut, rc::{Rc, Weak}};

type NodeRef<T> = Option<Rc<Node<T>>>;
type WeakNodeRef<T> = Weak<Node<T>>;

#[derive(Debug)]
pub struct Node<T> where T : PartialOrd {
    pub left   : NodeRef<T>,
    pub right  : NodeRef<T>,
    pub parent : WeakNodeRef<T>,
    pub value  : T
}

impl<T> Node<T> where T : PartialOrd {
    pub fn new(value : T) -> Self {
        Node { left: None,
               right: None,
               parent: Weak::new(),
               value }
    }

    
     fn left_insert(&mut self, val : T) {
         if let Some(left) = self.left.as_mut()  {
             left.borrow_mut().insert(val);
         } else {
             self.left = Some(Rc::new(Node::new(val)));
         }
    }

    fn right_insert(&mut self, val : T) {
        if let Some(right) = self.right.as_mut()  {
            right.insert(val);
        } else {
            self.right = Some(Rc::new(Node::new(val)));
        }
    }

    pub fn insert(&mut self, val : T) {
        if self.value <= val {
            self.right_insert(val)
        } else {
           self.left_insert(val);   
        }
    }
}


#[cfg(test)]
mod tree_test {
    use super::{Node};
    #[test]
    fn insert_right() {
        let mut node = Node::new(6);
        node.insert(8);
        assert_eq!(node.right.unwrap().value, 8);
    }
}