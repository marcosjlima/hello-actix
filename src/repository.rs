use super::models::{InputUser, NewUser, User};
use super::schema::tb_user::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::web;
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = tb_user.load::<User>(&conn)?;
    Ok(items)
}

pub fn db_get_user_by_id(pool: web::Data<Pool>,user_id: i64) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    tb_user.find(user_id).get_result::<User>(&conn)
}

pub fn add_single_user(db: web::Data<Pool>,item: web::Json<InputUser>) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        email: &item.email,
        nome: &item.nome,
        password: &item.password,
    };
    let res = insert_into(tb_user).values(&new_user).get_result(&conn)?;
    Ok(res)
}

pub fn delete_single_user(db: web::Data<Pool>,user_id: i64) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(tb_user.find(user_id)).execute(&conn)?;
    Ok(count)
}
