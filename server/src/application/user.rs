use domain::prelude::*;

pub struct UserAuthorizationService<L: LoadUserByNamePort, C: PasswordVerifierPort> {
    load_user_port: L,
    compare_password_port: C,
}

impl<L: LoadUserByNamePort, C: PasswordVerifierPort> UserAuthorizationService<L, C> {
    pub fn new(load_user_port: L, compare_password_port: C) -> Self {
        Self {
            load_user_port,
            compare_password_port,
        }
    }
}

impl<L: LoadUserByNamePort, C: PasswordVerifierPort> AuthorizeUserUseCase<L, C>
    for UserAuthorizationService<L, C>
{
    async fn authorize(
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
