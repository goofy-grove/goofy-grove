use crate::prelude::{AuthorizeUserCommand, DomainResult, LoadUserByNamePort, PasswordVerifierPort, User};

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
