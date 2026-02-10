use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainAuthorizationError {
    InvalidCredentials,
    UserNotFound,
}

pub trait AuthorizationUseCase {
    fn authorize(
        &self,
        command: AuthorizationCommand,
    ) -> impl Future<Output = DomainResult<User, DomainAuthorizationError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainRegistrationError {
    InternalError(String),
    FailedToHashPassword,
    UserAlreadyExists,
}

pub trait RegistrationUseCase {
    fn register(
        &self,
        command: RegistrationCommand,
    ) -> impl Future<Output = DomainResult<User, DomainRegistrationError>>;
}
