use actix_web::{App, HttpServer};
use web::handlers::{users::{find_user_by_id::find_user_by_id, create_user::handler::create_user}, hello::{hello_world::hello_world, hello_reply::hello_reply}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello_world)
        .service(hello_reply)
        .service(find_user_by_id)
        .service(create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}