use crate::prelude::{DomainResult, User, UserName, UserPassword};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadUserByNamePortError {
    InternalError(String),
    NotFound,
}

pub trait LoadUserByNamePort {
    fn load_user_by_name(&self, name: &UserName) -> impl Future<Output = DomainResult<User, LoadUserByNamePortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordVerifierPortError {
    InternalError(String),
    PasswordNotMatch,
}

pub trait PasswordVerifierPort {
    fn verify(
        &self,
        password: &UserPassword,
        hashed_password: &UserPassword,
    ) -> impl Future<Output = DomainResult<(), PasswordVerifierPortError>>;
}
