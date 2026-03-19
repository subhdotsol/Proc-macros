// for adding macros do cargo add macros
use macros::{ FromJson, ToJson };
use traits::{ JsonSerialize, JsonDeserialize };

#[derive(Debug, ToJson, FromJson)]
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
    //deserializing

    let struct_from_json = Person::from_json(&json_pair).unwrap();

    println!("{:?}", struct_from_json);
}

// what we want
// person.to_json();

// {
//     "id": 1,
//     "age": 20,
//     "name": "neha"
// }
