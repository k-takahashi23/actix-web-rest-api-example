use actix_web::{post, web, Responder};
use application::usecases::users::create_user::{usecase::CreateUserUsecase, input::CreateUserInput};
use domain::{repositories::users::UsersRepository};
use infrastructure::repositories::users::UsersRepositoryImpl;

use super::{request::CreateUserRequest, response::CreateUserResponse};

#[post("/users")]
pub async fn create_user(request: web::Json<CreateUserRequest>) -> impl Responder {
    let repository = UsersRepositoryImpl::new();
    let usecase = CreateUserUsecase(repository);
    let input = CreateUserInput::from(request.into_inner());
    let output = usecase.handle(input);
    CreateUserResponse::from(output)
}