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
    let user = User {
        name: "Fred Pi".to_string(),
        age: 33,
        email: "frr228@gmail.com".to_string(),
    };

    let firebase = Firebase::new("https://rust-firebase-default-rtdb.firebaseio.com/").unwrap();
}

async fn set_user() -> Response {}
