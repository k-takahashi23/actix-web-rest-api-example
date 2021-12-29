use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use domain::{entities::user::User, repositories::users::UsersRepository};
use infrastructure::repositories::users::UsersRepositoryImpl;
use serde::{Serialize};
use std::future::{Ready, ready};

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    name: String,
    age: i32
}

impl Responder for UserResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

#[get("/users/{user_id}")]
pub async fn find_user_by_id(web::Path(user_id): web::Path<String>) -> impl Responder {
    let users_repository = UsersRepositoryImpl {};
    let user = users_repository.find(user_id);
    UserResponse{ id: user.id.clone(), name: user.name.clone(), age: user.age }
}

#[post("/users")]
pub async fn create_user(request: web::Json<User>) -> impl Responder {
    let users_repository = UsersRepositoryImpl {};
    let user = User { id: request.id.clone(), name: request.name.clone(), age: request.age };
    users_repository.save(user);
    UserResponse{ id: request.id.clone(), name: request.name.clone(), age: request.age }
}