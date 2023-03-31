use dotenv::dotenv;
use firebase_rs::*; // importing firebase_rs library
use serde::{Deserialize, Serialize}; // importing serde serialization and deserialization traits
use std::{collections::HashMap, future}; // importing HashMap for storing data
use tokio;

// -- define User struct with name, age, and email fields
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: Option<String>,
    name: String,
    age: u32,
    email: String,
}

// -- define Response struct with name field
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    id: String,
}

// -- define async main function
#[tokio::main]
async fn main() {
    // -- mockup user
    let user = User {
        id: None,
        name: "Fred Pi".to_string(),
        age: 33,
        email: "someemail@gmail.com".to_string(),
    };

    let firebase_url = env::var("FIREBASE").unwrap();
    // -- create a new Firebase instance with the Firebase database URL
    let firebase = Firebase::new(&firebase_url).unwrap();

    let response = set_user(&firebase, &user).await;
    println!("{:?}", response.name);
    println!("{:?}", response.id);

    let mut user = get_user(&firebase, &response.id).await;

    let users = get_users(&firebase, &response.id).await;
    println!("{:?}", users);

    user.email = "anotheremail@gmail.com".to_String();

    let updated_user = update_user(&firebase, &response.id, &user).await;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.id).await;
}

// -- define async set_user function that sets user data in Firebase
async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    // -- get a reference to the "users" node in Firebase
    let firebase = firebase_client.at("users");
    // -- set the user data in Firebase
    let _users = firebase.set::<User>(&user).await;
    // -- return the response from Firebase as a Response struct
    return string_to_response(&_users.unwrap().data);
}

// -- define async get_users function that retrieves all users from Firebase
async fn get_users(firebase_client: &Firebase, user: &User) -> HashMap<String, User> {
    // -- get a reference to the "users" node in Firebase
    let firebase = firebase_client.at("users");
    // -- get all user data from Firebase as a HashMap
    let users = firebase.get::<HashMap<String, User>>().await;
    // -- print the retrieved user data
    println!("{:?}", users);
    // -- return the retrieved user data as a HashMap
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// -- helper functions

// -- convert string to response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// -- convert string to a User struct
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
