use std::fmt::{Display, Formatter};

struct A {
    a: i32,
}

impl A {
    fn new(a: i32) -> Self {
        A { a }
    }
}

struct B {
    b: i32,
}

impl B {
    fn new(b: i32) -> Self {
        B { b }
    }
}

enum Parent {
    ChildA(i32, A),
    ChildB(i32, B),
}

impl Display for Parent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Parent::ChildA(p, a) => write!(f, "{}, {}", p, a.a),
            Parent::ChildB(p, b) => write!(f, "{}, {}", p, b.b),
        }
    }
}

fn main() {
    let mut vec = Vec::new();

    let a = Parent::ChildA(10, A::new(20));
    let b = Parent::ChildB(30, B::new(40));

    vec.push(a);
    vec.push(b);

    let l_a = vec.get(0).unwrap();
    let l_b = vec.get(1).unwrap();

    println!("{}", l_a);
    println!("{}", l_b);
}
