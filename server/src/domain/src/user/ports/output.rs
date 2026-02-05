use crate::prelude::{DomainResult, User, UserName, UserPassword};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadUserByNamePortError {
    InternalError(String),
    NotFound,
}

pub trait LoadUserByNamePort {
    async fn load_user_by_name(&self, name: &UserName) -> DomainResult<User, LoadUserByNamePortError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordVerifierPortError {
    InternalError(String),
    PasswordNotMatch,
}

pub trait PasswordVerifierPort {
    async fn verify(
        &self,
        password: &UserPassword,
        hashed_password: &UserPassword,
    ) -> DomainResult<(), PasswordVerifierPortError>;
}
