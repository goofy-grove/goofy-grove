use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationError {
    InvalidCredentials,
    UserNotFound,
}

pub trait AuthorizationUseCase {
    fn authorize(
        &self,
        command: AuthorizationCommand,
    ) -> impl Future<Output = DomainResult<User, AuthorizationError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    InternalError(String),
    FailedToHashPassword,
    UserAlreadyExists,
}

pub trait RegistrationUseCase {
    fn register(
        &self,
        command: RegistrationCommand,
    ) -> impl Future<Output = DomainResult<User, RegistrationError>>;
}
