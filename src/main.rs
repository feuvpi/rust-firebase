use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    // -- mockup user
    let user = User {
        name: "Fred Pi".to_string(),
        age: 33,
        email: "frr228@gmail.com".to_string(),
    };

    let firebase = Firebase::new("https://rust-firebase-default-rtdb.firebaseio.com/").unwrap();
}

async fn set_user() -> Response {}

// -- helper functions

// -- convert string to response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// -- convert string to a User struct
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
