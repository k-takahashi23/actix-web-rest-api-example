use actix_web::{get, web, Responder};
use application::usecases::users::find_user_by_id::{input::FindUserByIdInput, usecase::FindUserByIdUsecase};
use domain::repositories::users::UsersRepository;
use infrastructure::repositories::users::UsersRepositoryImpl;
use super::{response::FindUserByIdResponse, request::FindUserByIdRequest};

#[get("/users/{user_id}")]
pub async fn find_user_by_id(request: web::Path<FindUserByIdRequest>) -> impl Responder {
    let repository = UsersRepositoryImpl::new();
    let usecase = FindUserByIdUsecase(repository);
    let input = FindUserByIdInput::from(request.into_inner());
    let output = usecase.handle(input);
    FindUserByIdResponse::from(output)
}