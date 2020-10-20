use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Stack<A: Clone> {
    Nil,
    Cons(A, Rc<Stack<A>>),
}

impl<A: Clone> Stack<A> {
    pub fn new() -> Self {
        Stack::Nil
    }

    pub fn push(&mut self, item: A) {
        let tail = std::mem::replace(self, Stack::Nil);
        let mut list = Stack::Cons(item, Rc::new(tail));
        std::mem::swap(self, &mut list)
    }

    pub fn pop(&mut self) -> Option<A> {
        let list = std::mem::replace(self, Stack::Nil);
        match list {
            Stack::Nil => None,
            Stack::Cons(elem, mut tail) => {
                std::mem::swap(self, Rc::make_mut(&mut tail));
                Some(elem)
            }
        }
    }
}
