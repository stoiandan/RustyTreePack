use std::process::id;

enum ChildType {
    Left,
    Right,
}

struct Node<T>
where
    T: Ord,
{
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    value: T,
}

impl<T> Node<T>
where
    T: Ord,
{
    pub fn new(value: T) -> Self {
        Node {
            left: None,
            right: None,
            parent: None,
            value,
        }
    }
}

pub struct Tree<T>
where
    T: Ord,
{
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Tree { nodes: Vec::new() }
    }


    pub fn insert(&mut self, val: T) {
        self.insert_at(0, val);
    }

    fn insert_at(&mut self, idx: usize, val: T)  {
            if self.nodes.len() <= idx {
                if idx == 0 {
                    self.nodes.push(Node::new(val));
                }
                return;
            }

            let len = self.nodes.len();
            let root = &mut self.nodes[idx];

            // right insert
            if val >= root.value {
                if let Some(right) = root.right {
                    self.insert_at(right, val);
                } else {
                    let mut new_node = Node::new(val);

                    new_node.parent = Some(idx);
                    root.right = Some(len);
                    self.nodes.push(new_node);
                }
            } else {
                if let Some(left) = root.left {
                    self.insert_at(left, val);
                } else {
                    let mut new_node = Node::new(val);

                    new_node.parent = Some(idx);
                    root.left = Some(len);
                    self.nodes.push(new_node);
                }
            }
    }


    pub fn find(&self, val: T) -> Option<&Node<T>> {
        self.find_at(0, val)
    }

    fn find_at(&self, idx: usize, val: T) -> Option<&Node<T>> {
                if self.nodes.len() <= idx {
                    return None;
                }

                let root = &self.nodes[idx];

                if root.value == val {
                    return Some(root);
                }

                if root.value >= val {
                    if let Some(left) = root.left {
                        self.find_at(left, val)
                    } else {
                        return None
                    }
                } else {
                    if let Some(right) = root.right {
                        self.find_at(right, val)
                    } else {
                        return None
                    }
                }
    }
}


#[cfg(test)]
mod tests {
    use super::Tree;
    #[test]
    fn insert_root() {
        let mut tree = Tree::new();

        tree.insert(5);

        assert_eq!(tree.find(5).unwrap().value, 5)
    }

    #[test]
    fn insert_left() {
        let mut tree = Tree::new();

        tree.insert(5);
        tree.insert(4);

       assert_eq!(tree.find(4).unwrap().value, 4);
    }

    #[test]
    fn insert_right() {
        let mut tree = Tree::new();

        tree.insert(5);
        tree.insert(7);

        assert_eq!(tree.find(7).unwrap().value,7);
    }

    #[test]
    fn insert_right_and_left() {
        let mut tree = Tree::new();

        tree.insert(5);
        tree.insert(7);
        tree.insert(3);

       assert_eq!(tree.find(3).unwrap().value,3);
    }

    #[test]
    fn insert_depth_left() {
        let mut tree = Tree::new();

        tree.insert(7);
        tree.insert(5);
        tree.insert(3);

       assert_eq!(tree.find(3).unwrap().value,3);
    }
}
