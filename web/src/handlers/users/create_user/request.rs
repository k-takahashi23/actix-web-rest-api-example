use application::usecases::users::create_user::input::CreateUserInput;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    age: i32,
}

impl From<CreateUserRequest> for CreateUserInput {
    fn from(from: CreateUserRequest) -> CreateUserInput {
        CreateUserInput { name: from.name.clone(), age: from.age }
    }
}