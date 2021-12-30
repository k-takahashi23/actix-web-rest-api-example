use domain::repositories::users::UsersRepository;

pub struct FindUserByIdUsecase<T: UsersRepository>(pub T);