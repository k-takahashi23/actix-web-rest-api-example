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

#[cfg(test)]
mod tests {
    use mockall::*;
    use domain::{entities::user::User, repositories::users::UsersRepository};
    use crate::usecases::users::create_user::{usecase::CreateUserUsecase, input::CreateUserInput, output::CreateUserOutput};

    mock! {
        pub UsersRepository {} 
    }

    #[test]
    fn create_user_ok() {
        // Given
        impl UsersRepository for MockUsersRepository {
            fn new() -> Self {
                MockUsersRepository {}
            }
        
            fn find(&self, _id: String) -> User {
                User {
                    id: "mock_id".to_string(),
                    name: "mock_name".to_string(),
                    age: 20,
                }
            }
        
            fn save(&self, _user: User) -> User {
                User {
                    id: "mock_id".to_string(),
                    name: "mock_name".to_string(),
                    age: 20,
                }
            }
        
            fn delete(&self, _id: String) -> User {
                User {
                    id: "mock_id".to_string(),
                    name: "mock_name_test".to_string(),
                    age: 20,
                }
            }   
        }
        let mock_repository = MockUsersRepository::new();
        let usecase = CreateUserUsecase(mock_repository);
        let input = CreateUserInput { name: "mock_name".to_string(), age: 20 };
        let expected = CreateUserOutput { id: "mock_id".to_string(), name: "mock_name".to_string(), age: 20 };

        // When
        let actual = usecase.handle(input);

        // Then
        assert_eq!(expected.id, actual.id);
        assert_eq!(expected.name, actual.name);
        assert_eq!(expected.age, actual.age);
    }
}