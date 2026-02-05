use crate::error::DomainError;
use crate::prelude::{DomainResult, User};
use crate::user::ports::{
    input::commands::AuthorizeUserCommand,
    output::{LoadUserByNamePort, PasswordVerifierPort},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainAuthorizationError {
    InvalidCredentials,
    UserNotFound,
}

pub struct AuthorizeUserUseCase<L: LoadUserByNamePort, C: PasswordVerifierPort> {
    load_user_port: L,
    compare_password_port: C,
}

impl<L: LoadUserByNamePort, C: PasswordVerifierPort> AuthorizeUserUseCase<L, C> {
    pub fn new(load_user_port: L, compare_password_port: C) -> Self {
        AuthorizeUserUseCase {
            load_user_port,
            compare_password_port,
        }
    }

    pub async fn authorize(
        &self,
        command: AuthorizeUserCommand,
    ) -> DomainResult<User, DomainAuthorizationError> {
        let user = self
            .load_user_port
            .load_user_by_name(command.name())
            .await
            .or(Err(DomainError::UseCaseError(
                DomainAuthorizationError::UserNotFound,
            )))?;

        self.compare_password_port
            .verify(command.password(), user.password())
            .await
            .map(|_| user)
            .or(Err(DomainError::UseCaseError(
                DomainAuthorizationError::InvalidCredentials,
            )))
    }
}
