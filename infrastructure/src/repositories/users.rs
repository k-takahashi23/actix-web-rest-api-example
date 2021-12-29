use domain::{entities::user::User, repositories::users::UsersRepository};

pub struct UsersRepositoryImpl {}

// Mock repository
impl UsersRepository for UsersRepositoryImpl {
    fn new() -> Self {
        UsersRepositoryImpl {}
    }

    fn find(&self, id: String) -> User {
        println!("find: user_id = {}", id);
        User {
            id,
            name: "mock_name".to_string(),
            age: 20,
        }
    }

    fn save(&self, user: User) -> User {
        println!("save: user_id = {}, name = {}, age = {}", user.id, user.name, user.age);
        user
    }

    fn delete(&self, id: String) -> User {
        println!("delete: user_id = {}", id);
        User {
            id,
            name: "mock_name".to_string(),
            age: 20,
        }
    }
}