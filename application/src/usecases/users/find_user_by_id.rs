use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use domain::repositories::users::UsersRepository;
use serde::{Serialize};
use std::future::{Ready, ready};

pub struct FindUserByIdRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct FindUserByIdResponse {
    id: String,
    name: String,
    age: i32
}

impl Responder for FindUserByIdResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

pub struct FindUserByIdUsecase<T: UsersRepository>(pub T);

impl<T: UsersRepository> FindUserByIdUsecase<T> {
    pub fn handle(&self, request: FindUserByIdRequest) -> FindUserByIdResponse {
        let users_repository = T::new();
        let user = users_repository.find(request.id);
        FindUserByIdResponse{ id: user.id, name: user.name, age: user.age }
    }
}