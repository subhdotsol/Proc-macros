// for adding macros do cargo add macros
use macros::ToJson;
use traits::JsonSerialize;

#[derive(ToJson)]
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

    // println!("{:#?}", person);

    let json_pair = person.to_json();
    println!("{}", json_pair);
}

// what we want
// person.to_json();

// {
//     "id": 1,
//     "age": 20,
//     "name": "neha"
// }
