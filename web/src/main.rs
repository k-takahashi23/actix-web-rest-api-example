use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(application::services::hello::hello_world)
            .service(application::services::users::find_user_by_id)
            .service(application::services::users::create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}