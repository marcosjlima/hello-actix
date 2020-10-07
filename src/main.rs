use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate serde;
use serde::{Deserialize, Serialize};

const CONTEXT_NAME: &str = "app";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server ON!");

    HttpServer::new(|| {
        App::new().service(
            web::scope(CONTEXT_NAME)
                .service(hello)
                .service(echo)
                .service(hey),
        )
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/hey")]
async fn hey() -> impl Responder {
    let point1: Point = Point { x: 1.0, y: 2.0 };
    let point2: Point = Point { x: 3.0, y: 4.0 };
    let length = ((point1.x - point2.x) * (point1.x - point2.x)
        + (point1.y - point2.y) * (point1.y - point2.y))
        .sqrt();
    let valid = if length == 0.0 { false } else { true };
    let line = Line {
        points: vec![point1, point2],
        valid: valid,
        length: length,
        desc: "a thin line".to_string(),
    };
    let lines = serde_json::to_string(&line).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(lines)
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
    points: Vec<Point>,
    valid: bool,
    length: f32,
    desc: String,
}
