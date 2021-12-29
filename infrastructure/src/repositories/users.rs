use domain::{entities::user::User, repositories::users::UsersRepository};

pub struct UsersRepositoryImpl {}

// Mock repository
impl UsersRepository for UsersRepositoryImpl {
    fn find(&self, id: String) -> User {
        User {
            id,
            name: "mock_name".to_string(),
            age: 20,
        }
    }

    fn save(&self, user: User) {
        println!("save: user_id = {}, name = {}, age = {}", user.id, user.name, user.age);
    }

    fn delete(&self, id: String) {
        println!("delete: user_id = {}", id);
    }
}