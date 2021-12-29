use actix_web::{post, web, Error, HttpRequest, HttpResponse, Responder};
use domain::{entities::user::User, repositories::users::UsersRepository};
use infrastructure::repositories::users::UsersRepositoryImpl;
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    age: i32
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    id: String,
    name: String,
    age: i32
}

impl Responder for CreateUserResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

#[post("/users")]
pub async fn create_user(request: web::Json<CreateUserRequest>) -> impl Responder {
    let users_repository = UsersRepositoryImpl {};
    let user = User { id: "mock_id".to_string(), name: request.name.clone(), age: request.age };
    users_repository.save(user);
    CreateUserResponse{ id: "mock_id".to_string(), name: request.name.clone(), age: request.age }
}