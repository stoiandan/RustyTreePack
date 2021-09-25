use std::rc::{Rc, Weak};
use std::cell::{RefCell};

type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> where T : PartialOrd {
     left   : NodeRef<T>,
     right  : NodeRef<T>,
     parent : WeakNodeRef<T>,
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
             let rc = &Rc::new(RefCell::new(Node::new(val)));
             self.left = Some(rc.clone());
             self.left.as_ref().unwrap().borrow_mut().parent  = Rc::downgrade(&rc.clone());
            }
    }

    fn right_insert(&mut self, val : T) {
        if let Some(right) = self.right.as_mut()  {
            right.borrow_mut().insert(val);
        } else {
            let rc = &Rc::new(RefCell::new(Node::new(val)));
            self.right = Some(rc.clone());
            self.right.as_ref().unwrap().borrow_mut().parent  = Rc::downgrade(&rc.clone());
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
    use std::ptr;
    #[test]
    fn insert_right() {
        let mut node = Node::new(6);
        node.insert(8);
        assert_eq!(node.right.as_mut().unwrap().borrow_mut().value, 8);
    }

    #[test]
    fn insert_left() {
        let mut node = Node::new(6);
        node.insert(3);
        assert_eq!(node.left.as_mut().unwrap().borrow_mut().value, 3);
    }

    #[test]
    fn test_parent() {
        let mut node = Node::new(5);
        node.insert(6);
        print!("{:p}\n{:p}",&node,&*node.right.as_ref().unwrap().borrow().parent.upgrade().as_ref().unwrap().borrow());
        assert!(ptr::eq(&node,&*node.right.as_ref().unwrap().borrow().parent.upgrade().as_ref().unwrap().borrow()));
    }
}