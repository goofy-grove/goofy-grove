use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AuthorizationError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,
}

pub trait AuthorizationUseCase {
    fn authorize(
        &self,
        command: AuthorizationCommand,
    ) -> impl Future<Output = DomainResult<User, AuthorizationError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistrationError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Failed to hash password")]
    FailedToHashPassword,

    #[error("User already exists")]
    UserAlreadyExists,
}

pub trait RegistrationUseCase {
    fn register(
        &self,
        command: RegistrationCommand,
    ) -> impl Future<Output = DomainResult<User, RegistrationError>>;
}
