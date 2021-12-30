use domain::{repositories::users::UsersRepository, entities::user::User};
use super::{input::CreateUserInput, output::CreateUserOutput, usecase::CreateUserUsecase};

impl<T: UsersRepository> CreateUserUsecase<T> {
    pub fn handle(&self, input: CreateUserInput) -> CreateUserOutput {
        let users_repository = T::new();
        let user = User::new(input.name.clone(), input.age );
        let saved_user = users_repository.save(user);
        CreateUserOutput{ id: saved_user.id, name: saved_user.name, age: saved_user.age }
    }
}