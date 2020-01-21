use math::{addition, multiplication};

fn main() {
    println!("{}", addition::integers::apply(3, 1));
    println!("{}", addition::floats::apply(2.5, 3.1));
    println!("{}", multiplication::integers::apply(3, 1));
    println!("{}", multiplication::floats::apply(2.5, 3.1));
}
