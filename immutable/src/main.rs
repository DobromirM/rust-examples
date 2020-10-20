use crate::stack::Stack;
use crate::tree::Tree;
use rand::seq::SliceRandom;

mod stack;
mod tree;

fn main() {
    println!("------Stack------");
    stack();
    println!("------Tree------");
    tree();
}

fn stack() {
    let mut stack = Stack::new();

    for i in 0..10 {
        stack.push(CloneTracker(i));
    }

    // Clones everything
    let _clone = stack.clone();

    for i in (0..10).rev() {
        // Now they need to be cloned because we have reference above
        assert_eq!(stack.pop(), Some(CloneTracker(i)));
    }
}

fn tree() {
    let mut tree = Tree::new();

    let mut numbers: Vec<u32> = (0..50).map(|x| x * 2).collect();
    numbers.shuffle(&mut rand::thread_rng());

    for num in numbers.clone() {
        tree.insert(CloneTracker(num));
    }

    // Clones everything
    let _clone = tree.clone();

    // Clones only the part that is changed
    tree.insert(CloneTracker(47));
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct CloneTracker(u32);

impl Clone for CloneTracker {
    fn clone(&self) -> Self {
        println!("Cloning {}", self.0);
        CloneTracker(self.0)
    }
}
