use crate::prelude::{DomainResult, User, UserId, UserName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadUserByNamePortError {
    InternalError(String),
    NotFound,
}

pub trait LoadUserByNamePort {
    fn load_user_by_name(
        &self,
        name: &UserName,
    ) -> impl Future<Output = DomainResult<User, LoadUserByNamePortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveUserPortError {
    InternalError(String),
    UserAlreadyExists,
}

pub trait SaveUserPort {
    fn save_user(&self, user: &User)
    -> impl Future<Output = DomainResult<User, SaveUserPortError>>;
}

pub trait UserIdGenerator {
    fn generate(&self) -> UserId;
}
