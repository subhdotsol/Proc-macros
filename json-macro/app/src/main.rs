// for adding macros do cargo add macros
use macros::ToJson;

#[derive(Debug, ToJson)]
struct Person {
    id: i32,
    age: i32,
    name: String,
}

fn main() {
    let person = Person {
        id: 1,
        age: 20,
        name: String::from("neha"),
    };

    println!("{:#?}", person);
}

// what we want
// person.to_json();

// {
//     "id": 1,
//     "age": 20,
//     "name": "neha"
// }
