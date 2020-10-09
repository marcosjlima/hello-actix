#[macro_use]
extern crate diesel;
use actix_web::{dev::ServiceRequest, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod user;
mod models;
mod schema;
mod repository;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Server ON!");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(user::get_users)
            .service(user::get_users_by_id)
            .service(user::add_users)
            .service(user::delete_users_by_id)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}