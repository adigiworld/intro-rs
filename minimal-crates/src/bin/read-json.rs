use serde::{Deserialize, Serialize};
use serde_json;

fn main() {
    println!("Reading json data");

    let raw_json = r#"
    {
        "name" : "Ali",
        "age" : 25,
        "gender" : "Male",
        "address" : {
            "name" : "Amroha",
            "postal_code" : 244221,
            "state" : "UP"
        }
    }
    "#;

    let p: Person = serde_json::from_str(raw_json).unwrap();

    println!("Person data : \n{p:#?}");
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    gender: String,
    // gender: Gender,
    address: Address,
}

// #[derive(Debug, Serialize, Deserialize)]
// enum Gender {
//     Male(String),
//     Female(String),
// }

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    name: String,
    postal_code: u32,
    state: String,
}
