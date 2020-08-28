#[macro_use]
extern crate macro_derive;

fn main() {
    Foo::test();
}

trait MacroTrait {
    fn test();
}

#[derive(MacroTrait)]
struct Foo;
