#[macro_use]
extern crate macro_attr;

fn main() {
    Foo::test()
}

#[test_macro_attr(custom_func)]
struct Foo;


fn custom_func() {
    println!("Attached function")
}

trait MacroTrait {
    fn test();
}
