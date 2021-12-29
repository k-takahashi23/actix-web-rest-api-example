use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use domain::{repositories::users::UsersRepository, entities::user::User};
use std::future::{Ready, ready};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: i32
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

pub struct CreateUserUsecase<T: UsersRepository>(pub T);

impl<T: UsersRepository> CreateUserUsecase<T> {
    pub fn handle(&self, request: CreateUserRequest) -> CreateUserResponse {
        let users_repository = T::new();
        let user = User::new(request.name.clone(), request.age );
        let saved_user = users_repository.save(user);
        CreateUserResponse{ id: saved_user.id, name: saved_user.name, age: saved_user.age }
    }
}