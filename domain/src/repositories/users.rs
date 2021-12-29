use crate::entities::user::User;

pub trait UsersRepository {
  fn find(&self, id: String) -> User;
  fn save(&self, user: User) -> User;
  fn delete(&self, id: String) -> User;
}