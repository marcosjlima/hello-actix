use actix_web::{App, HttpServer};

mod user;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    println!("Server ON!");

    HttpServer::new(|| {
        App::new()
            .service(user::get_users)
            .service(user::get_users_by_id)
            .service(user::add_users)
            .service(user::delete_users_by_id)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}