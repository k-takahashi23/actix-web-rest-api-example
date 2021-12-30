use actix_web::{get, web, Responder};
use application::usecases::users::find_user_by_id::{input::FindUserByIdInput, usecase::FindUserByIdUsecase};
use domain::repositories::users::UsersRepository;
use infrastructure::repositories::users::UsersRepositoryImpl;


use super::response::FindUserByIdResponse;

#[get("/users/{user_id}")]
pub async fn find_user_by_id(web::Path(user_id): web::Path<String>) -> impl Responder {
    let repository = UsersRepositoryImpl::new();
    let usecase = FindUserByIdUsecase(repository);
    let output = usecase.handle(FindUserByIdInput { id: user_id });
    FindUserByIdResponse::from(output)
}