use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use application::usecases::users::create_user::output::CreateUserOutput;
use std::future::{Ready, ready};
use serde::{Serialize};

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