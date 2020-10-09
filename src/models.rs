use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub nome: String,
    pub password: String,
}

#[derive(Insertable, Debug)]
#[table_name = "tb_user"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub nome: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub nome: String,
    pub password: String,
}