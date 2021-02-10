use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    let first = "First".to_string();
    let second = "Second".to_string();
    let third = "Second".to_string();

    let first_ptr: *const String = &first;
    // let first_ptr = &first;

    put(&mut map, first);
    unsafe {
        eprintln!("first_ptr = {:#?}", *first_ptr);
    }
}

fn put(map: &mut HashMap<String, i32>, str: String) {
    map.insert(str, 1);
}
