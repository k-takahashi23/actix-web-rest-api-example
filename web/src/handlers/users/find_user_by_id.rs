use actix_web::{get, web, Error, HttpRequest, HttpResponse,Responder};
use application::usecases::users::find_user_by_id::{input::FindUserByIdInput, usecase::FindUserByIdUsecase};
use domain::repositories::users::UsersRepository;
use infrastructure::repositories::users::UsersRepositoryImpl;
use std::future::{Ready, ready};
use serde::{Serialize};

#[derive(Serialize)]
pub struct FindUserByIdResponse {
    id: String,
    name: String,
    age: i32,
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
    let repository = UsersRepositoryImpl::new();
    let usecase = FindUserByIdUsecase(repository);
    let output = usecase.handle(FindUserByIdInput { id: user_id });
    FindUserByIdResponse { id: output.id.clone(), name: output.name.clone(), age: output.age }
}