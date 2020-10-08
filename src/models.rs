use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(
        id: i32, first_name: String, last_name: String,
        email: String, created_at: chrono::NaiveDateTime,
    ) -> User {
        User { id, first_name, last_name, email, created_at }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}