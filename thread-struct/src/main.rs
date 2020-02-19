use std::{thread, time};
use std::sync::Arc;

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Person {
        return Person { name, age };
    }

    fn print_name(&self) {
        println!("Name: {}", self.name);
    }

    fn print_age(&self) {
        println!("Age: {}", self.age);
    }
}


fn main() {
    let person = Arc::new(Person::new(String::from("John"), 32));
    let cloned_person = person.clone();
    thread::spawn(move || cloned_person.print_name());
    let cloned_person = person.clone();
    thread::spawn(move || cloned_person.print_age());

    thread::sleep(time::Duration::from_secs(10));
}

