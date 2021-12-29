use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(application::usecases::hello::hello_world::hello_world)
        .service(application::usecases::hello::hello_reply::hello_reply)
        .service(application::usecases::users::find_user_by_id::find_user_by_id)
        .service(application::usecases::users::create_user::create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}