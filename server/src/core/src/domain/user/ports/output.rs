use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoadUserByNamePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("User not found")]
    NotFound,
}

pub trait LoadUserByNamePort {
    fn load_user_by_name(
        &self,
        name: &Username,
    ) -> impl Future<Output = Result<User, LoadUserByNamePortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SaveUserPortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("User already exists")]
    UserAlreadyExists,
}

pub trait SaveUserPort {
    fn save_user(&self, user: User) -> impl Future<Output = Result<User, SaveUserPortError>>;
}
