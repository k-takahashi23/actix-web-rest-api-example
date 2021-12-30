use domain::repositories::users::UsersRepository;
use super::{output::FindUserByIdOutput, input::FindUserByIdInput, usecase::FindUserByIdUsecase};

impl<T: UsersRepository> FindUserByIdUsecase<T> {
    pub fn handle(&self, request: FindUserByIdInput) -> FindUserByIdOutput {
        let users_repository = T::new();
        let user = users_repository.find(request.id);
        FindUserByIdOutput{ id: user.id, name: user.name, age: user.age }
    }
}