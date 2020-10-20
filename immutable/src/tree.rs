use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Tree<A: Ord> {
    Leaf,
    Node {
        left: Rc<Tree<A>>,
        item: A,
        right: Rc<Tree<A>>,
    },
}

impl<A: Ord + Clone> Tree<A> {
    pub fn new() -> Self {
        Tree::Leaf
    }

    pub fn singleton(item: A) -> Self {
        Tree::Node {
            left: Rc::new(Tree::Leaf),
            item,
            right: Rc::new(Tree::Leaf),
        }
    }

    pub fn insert(&mut self, item: A) {
        match self {
            Tree::Leaf => *self = Tree::singleton(item),
            Tree::Node {
                left,
                item: node_item,
                right,
            } => {
                if item < *node_item {
                    Rc::make_mut(left).insert(item)
                } else if item > *node_item {
                    Rc::make_mut(right).insert(item)
                }
            }
        }
    }

    pub fn find(&self, item: &A) -> bool {
        match self {
            Tree::Leaf => false,
            Tree::Node {
                left,
                item: node_item,
                right,
            } => {
                if item < node_item {
                    left.find(item)
                } else if item > node_item {
                    right.find(item)
                } else {
                    true
                }
            }
        }
    }
}
