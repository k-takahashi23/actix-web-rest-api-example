use serde::{Deserialize};

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: i32
}