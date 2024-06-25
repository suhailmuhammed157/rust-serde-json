use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    name: String,
    mobile: String,
    address: Vec<Address>,
}

fn read_raw_json(raw_data: &str) -> Result<UserData, Box<dyn Error>> {
    let parsed: UserData = serde_json::from_str(raw_data)?;
    Ok(parsed)
}

fn main() {
    let json_data = r#"
    {
    "name":"Suhail",
    "mobile":"0568037393",
    "address":[
            {
                "street":"Abc street",
                "zip":"0000"
            },
            {
                "street":"Abcd street",
                "zip":"0001"
            }
        ]
    }
    "#;

    let parsed_data: UserData = read_raw_json(json_data).unwrap();
    println!("{:?}", parsed_data);
}
