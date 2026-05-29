use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AuthenticationError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,
}

pub trait AuthenticationUseCase {
    fn authenticate(
        &self,
        command: AuthenticationCommand,
    ) -> impl Future<Output = Result<User, AuthenticationError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistrationError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Failed to hash password")]
    FailedToHashPassword,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub trait RegistrationUseCase {
    fn register(
        &self,
        command: RegistrationCommand,
    ) -> impl Future<Output = Result<User, RegistrationError>>;
}
