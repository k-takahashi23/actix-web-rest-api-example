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
    use domain::{repositories::users::UsersRepository, entities::user::User};
    use crate::usecases::users::create_user::{usecase::CreateUserUsecase, input::CreateUserInput, output::CreateUserOutput};

    // TODO: Mock ライブラリ使う
    pub struct MockUsersRepository {}
    impl UsersRepository for MockUsersRepository {
        fn new() -> Self {
            MockUsersRepository {}
        }
    
        fn find(&self, id: String) -> User {
            User {
                id,
                name: "mock_name_test".to_string(),
                age: 30,
            }
        }
    
        fn save(&self, user: User) -> User {
            user
        }
    
        fn delete(&self, id: String) -> User {
            User {
                id,
                name: "mock_name_test".to_string(),
                age: 20,
            }
        }   
    }

    #[test]
    fn create_user_ok() {
        // Given
        let repository = MockUsersRepository::new();
        let usecase = CreateUserUsecase(repository);
        let input = CreateUserInput { name: "mock_name_test".to_string(), age: 30 };
        let expected = CreateUserOutput { id: "1".to_string(), name: "mock_name_test".to_string(), age: 30 };

        // When
        let actual = usecase.handle(input);

        // Then
        // TODO: uuid のモック化
        // assert_eq!(expected.id, actual.id);
        assert_eq!(expected.name, actual.name);
        assert_eq!(expected.age, actual.age);
    }
}