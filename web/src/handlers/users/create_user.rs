use actix_web::{post, web, Responder};
use application::usecases::users::create_user::{CreateUserRequest, CreateUserUsecase};
use domain::{repositories::users::UsersRepository};
use infrastructure::repositories::users::UsersRepositoryImpl;

#[post("/users")]
pub async fn create_user(request: web::Json<CreateUserRequest>) -> impl Responder {
    let repository = UsersRepositoryImpl::new();
    let usecase = CreateUserUsecase(repository);
    usecase.handle(CreateUserRequest { name: request.name.clone(), age: request.age })
}