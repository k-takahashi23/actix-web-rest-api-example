use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello/{reply_name}")]
pub async fn hello_reply(web::Path(reply_name): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", reply_name))
}