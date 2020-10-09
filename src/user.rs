use actix_web::{delete, get, post, web, Error, HttpResponse};
use super::models::InputUser;
use super::repository;
use super::Pool;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || repository::get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/users/{id}")]
pub async fn get_users_by_id(db: web::Data<Pool>, user_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || repository::db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[post("/users")]
pub async fn add_users(db: web::Data<Pool>,item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || repository::add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[delete("/users/{id}")]
pub async fn delete_users_by_id(db: web::Data<Pool>,user_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || repository::delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}