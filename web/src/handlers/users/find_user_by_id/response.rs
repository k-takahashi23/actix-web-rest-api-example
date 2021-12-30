use actix_web::{Error, HttpRequest, HttpResponse,Responder};
use application::usecases::users::find_user_by_id::{output::FindUserByIdOutput};
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

impl From<FindUserByIdOutput> for FindUserByIdResponse {
    fn from(from: FindUserByIdOutput) -> FindUserByIdResponse {
        FindUserByIdResponse { id: from.id.clone(), name: from.name.clone(), age: from.age }
    }
}