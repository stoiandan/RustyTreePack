use std::{ops::Index, result};


#[derive(Debug, Clone)] 
struct NoNodeErr;


type  Result<T> = result::Result<Option<T>, NoNodeErr>;

pub struct Tree<T> where T : PartialOrd {
    nodes: Vec<Option<T>>
}

impl<T> Tree<T> where T : PartialOrd {
    
    pub fn new() -> Self {
            Tree {
                nodes : Vec::new()
            }
    }

    pub fn insert_into(&mut self, val : T) {
        self.insert(0, val);
    }

    fn insert(&mut self, idx: usize, val : T) {
        match self.get_at(idx) {
            Ok(None) =>  self.nodes[0] = Some(val),
            Ok(Some(t)) => {
                // insert to the left
                if t >= &val {
                    self.insert(idx*2, val)
                } else { // insert to the right
                    self.insert((idx*2)+1, val)
                }
            },
            _ => self.nodes.push(Some(val))
        };
    }

    fn insert_left(&mut self, val : T) {
        self.nodes.push(Some(val));
        self.nodes.push(None);
    }

    fn insert_right(&mut self, val : T) {
        self.nodes.push(None);
        self.nodes.push(Some(val));
    }


    fn get_left(&self, idx: usize) -> Result<&T> {
        self.get_at(idx* 2)
    }

    fn get_right(&self, idx: usize) -> Result<&T> {
        self.get_at((idx* 2) + 1)
    }

    fn get_at(&self, idx: usize) -> Result<&T> {
        if idx >= self.nodes.len() {
            Err(NoNodeErr)
        } else {
            Ok(self.nodes[idx].as_ref())
        }
    }

    fn root(&self) -> Result<&T> {
        self.get_at(0)
    }
}


impl<T> Index<usize> for Tree<T> where T : PartialOrd {
    type Output = T;

    fn index<'a>(&'a self, i: usize) -> &T {
        self.nodes[i].as_ref().unwrap()
    }
}



#[cfg(test)]
mod tests {
    use  super::Tree;
    #[test]
    fn insert_root() {
        let mut tree = Tree::new();

        tree.insert_into(5);

        assert_eq!(tree[0],5)
    }
}