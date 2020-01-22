macro_rules! length {

    ($v:expr) => (1);

    ($v:expr, $($vs:expr),+) => (
        length!($v) + length!($($vs),+);
    );

    (vec $v:expr) => ($v.len());
}

fn main() {
    let numbers = vec![1, 2, 3, 4];

    println!("{}", length!(10));
    println!("{}", length!(10, 15));
    println!("{}", length!(10, 15, 20));
    println!("{}", length!(10, 15, 20, 25));
    println!("{}", length!(10, 15, 20, 25, 30));
    println!("{}", length!(10, 15, 20, 25, 30, 35));
    println!("Numbers length: {}", length!(vec numbers));
}

