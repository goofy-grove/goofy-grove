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

pub trait AuthorizeUserUseCase<L: LoadUserByNamePort, C: PasswordVerifierPort> {
    fn authorize(
        &self,
        command: AuthorizeUserCommand,
    ) -> impl Future<Output = DomainResult<User, DomainAuthorizationError>>;
}
