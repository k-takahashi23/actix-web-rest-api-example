use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use domain::repositories::users::UsersRepository;
use infrastructure::repositories::users::UsersRepositoryImpl;
use serde::{Serialize};
use std::future::{Ready, ready};

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

#[get("/users/{user_id}")]
pub async fn find_user_by_id(web::Path(user_id): web::Path<String>) -> impl Responder {
    let users_repository = UsersRepositoryImpl {};
    let user = users_repository.find(user_id);
    FindUserByIdResponse{ id: user.id.clone(), name: user.name.clone(), age: user.age }
}