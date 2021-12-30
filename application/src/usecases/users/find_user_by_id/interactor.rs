use domain::repositories::users::UsersRepository;
use super::{output::FindUserByIdOutput, input::FindUserByIdInput, usecase::FindUserByIdUsecase};

impl<T: UsersRepository> FindUserByIdUsecase<T> {
    pub fn handle(&self, request: FindUserByIdInput) -> FindUserByIdOutput {
        let users_repository = T::new();
        let user = users_repository.find(request.id);
        FindUserByIdOutput{ id: user.id, name: user.name, age: user.age }
    }
}

#[cfg(test)]
mod tests {
    use mockall::*;
    use domain::{repositories::users::UsersRepository, entities::user::User};
    use crate::usecases::users::find_user_by_id::{usecase::FindUserByIdUsecase, input::FindUserByIdInput, output::FindUserByIdOutput};

    mock! {
        pub UsersRepository {} 
    }

    #[test]
    fn find_user_by_id_ok() {
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
        let usecase = FindUserByIdUsecase(mock_repository);
        let input = FindUserByIdInput { id: "mock_id".to_string() };
        let expected = FindUserByIdOutput { id: "mock_id".to_string(), name: "mock_name".to_string(), age: 20 };

        // When
        let actual = usecase.handle(input);

        // Then
        assert_eq!(expected.id, actual.id);
        assert_eq!(expected.name, actual.name);
        assert_eq!(expected.age, actual.age);
    }
}