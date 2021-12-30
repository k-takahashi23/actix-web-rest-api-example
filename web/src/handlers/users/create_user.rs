use actix_web::{post, web, Error, HttpRequest, HttpResponse, Responder};
use application::usecases::users::create_user::{usecase::CreateUserUsecase, input::CreateUserInput, output::CreateUserOutput};
use domain::{repositories::users::UsersRepository};
use infrastructure::repositories::users::UsersRepositoryImpl;
use std::future::{Ready, ready};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    age: i32,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    id: String,
    name: String,
    age: i32,
}

impl Responder for CreateUserResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

impl From<CreateUserOutput> for CreateUserResponse {
    fn from(from: CreateUserOutput) -> CreateUserResponse {
        CreateUserResponse { id: from.id.clone(), name: from.name.clone(), age: from.age }
    }
}

#[post("/users")]
pub async fn create_user(request: web::Json<CreateUserRequest>) -> impl Responder {
    let repository = UsersRepositoryImpl::new();
    let usecase = CreateUserUsecase(repository);
    let output = usecase.handle(CreateUserInput { name: request.name.clone(), age: request.age });
    CreateUserResponse::from(output)
}