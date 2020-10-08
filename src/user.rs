use crate::models::{User, InputUser};
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let mut users: Vec<User> = Vec::new();

    for i in 1..8 {
        users.push(User::new(
            i, String::from("Marcos"), String::from(format!("Lima {}",i)), 
            String::from(format!("marcos_{}@lima.com", i)),
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd(2015, 6, 3),
                chrono::NaiveTime::from_hms_milli(12, 34, 56, 789),
            ),
        ));
    }

    HttpResponse::Ok().json(users)
}

#[get("/users/{id}")]
pub async fn get_users_by_id(user_id: web::Path<i32>) -> impl Responder {
    let id: i32 = *user_id;
    let user = User::new(
        id, String::from("Marcos"), String::from(format!("Lima {}", user_id)), 
        String::from(format!("marcos_{}@lima.com", user_id)),
        chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd(2015, 6, 3),
            chrono::NaiveTime::from_hms_milli(12, 34, 56, 789),
        ),
    );

    HttpResponse::Ok().json(user)
}

#[post("/users")]
pub async fn add_users(user: web::Json<InputUser>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", user))
}

#[delete("/users/{id}")]
pub async fn delete_users_by_id(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::NoContent()
}
