pub mod usecases;

#[cfg(test)]
mod tests {
    use domain::{repositories::users::UsersRepository, entities::user::User};
    use crate::usecases::users::{find_user_by_id::{usecase::FindUserByIdUsecase, input::FindUserByIdInput, output::FindUserByIdOutput}, create_user::{usecase::CreateUserUsecase, input::CreateUserInput, output::CreateUserOutput}};

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
    fn find_user_by_id_ok() {
        // Given
        let repository = MockUsersRepository::new();
        let usecase = FindUserByIdUsecase(repository);
        let input = FindUserByIdInput { id: "1".to_string() };
        let expected = FindUserByIdOutput { id: "1".to_string(), name: "mock_name_test".to_string(), age: 30 };

        // When
        let actual = usecase.handle(input);

        // Then
        assert_eq!(expected.id, actual.id);
        assert_eq!(expected.name, actual.name);
        assert_eq!(expected.age, actual.age);
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