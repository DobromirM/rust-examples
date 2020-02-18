use std::cell::RefCell;

#[derive(Debug)]
struct Name {
    text: String
}

impl Name {
    fn new(text: String) -> Name {
        return Name { text };
    }
}

struct Person {
    name: RefCell<Name>,
    age: i32,
}


impl Person {
    fn new(name: String, age: i32) -> Person {
        return Person { name: RefCell::new(Name::new(name)), age };
    }

    fn set_name(&self, name: String) {
        self.name.borrow_mut().text = name;
    }

    fn print_name(&self) {
        println!("Name: {}", self.name.borrow().text);
    }

    fn print_age(&self) {
        println!("Age: {}", self.age);
    }
}

fn main() {
    let person = Person::new(String::from("Foo"), 32);
    person.print_name();
    person.print_age();
    person.set_name(String::from("Bar"));
    person.print_name();
    person.print_age();
}
