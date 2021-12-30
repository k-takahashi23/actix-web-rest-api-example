use application::usecases::users::find_user_by_id::input::FindUserByIdInput;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct FindUserByIdRequest {
    user_id: String,
}

impl From<FindUserByIdRequest> for FindUserByIdInput {
    fn from(from: FindUserByIdRequest) -> FindUserByIdInput {
        FindUserByIdInput { id: from.user_id }
    }
}