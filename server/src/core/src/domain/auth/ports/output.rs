use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PasswordVerifierPortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Password not match")]
    PasswordNotMatch,
}

pub trait PasswordVerifierPort {
    fn verify(
        &self,
        proposed_password: &Secret,
        confirmed_password: &Secret,
    ) -> impl Future<Output = DomainResult<(), PasswordVerifierPortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PasswordHasherPortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait PasswordHasherPort {
    fn hash(
        &self,
        password: &Secret,
    ) -> impl Future<Output = DomainResult<Secret, PasswordHasherPortError>>;
}
