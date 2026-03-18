use macros::ToMap;
use std::collections::HashMap;
use traits::ToMap;

#[derive(Debug, ToMap)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person {
        name: String::from("usashi"),
        age: 34,
    };
    println!("{:?}", person);

    // let mut person_hashmap = HashMap::new();
    // person_hashmap.insert("name", person.name);
    // person_hashmap.insert("age", person.age.to_string());

    // let mut person_hashmap = person.to_map(); // after building my macro

    println!("{:?}", person);
}

// what do we want ?
// person.to_map();
// {"name": "usashi", "age": "34"}
