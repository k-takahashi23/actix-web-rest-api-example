use domain::{repositories::users::UsersRepository};

pub struct CreateUserUsecase<T: UsersRepository>(pub T);