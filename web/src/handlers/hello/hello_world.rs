use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}