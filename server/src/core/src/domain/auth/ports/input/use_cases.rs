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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidateTokensError {
    InternalError(String),
    TokensDoNotMatch,
}

pub trait ValidateTokenUseCase {
    fn compare_tokens(
        &self,
        command: ValidateTokenCommand,
    ) -> impl Future<Output = DomainResult<TokenData, ValidateTokensError>>;
}
