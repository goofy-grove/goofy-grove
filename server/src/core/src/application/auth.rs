use crate::domain::prelude::*;

#[derive(Debug, Clone)]
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

impl<L: LoadUserByNamePort, C: PasswordVerifierPort> AuthorizationUseCase
    for UserAuthorizationService<L, C>
{
    async fn authorize(
        &self,
        command: AuthorizationCommand,
    ) -> DomainResult<User, AuthorizationError> {
        let user = self
            .load_user_port
            .load_user_by_name(command.name())
            .await
            .or(Err(DomainError::UseCaseError(
                AuthorizationError::UserNotFound,
            )))?;

        self.compare_password_port
            .verify(
                command.secret(),
                &Secret::new(user.password().value().to_owned()),
            )
            .await
            .map(|_| user)
            .or(Err(DomainError::UseCaseError(
                AuthorizationError::InvalidCredentials,
            )))
    }
}

#[derive(Debug, Clone)]
pub struct RegistrationService<S: SaveUserPort, H: PasswordHasherPort, U: IdGenerator> {
    save_user_port: S,
    hash_password_port: H,
    id_generator: U,
}

impl<S: SaveUserPort, H: PasswordHasherPort, U: IdGenerator> RegistrationService<S, H, U> {
    pub fn new(save_user_port: S, hash_password_port: H, id_generator: U) -> Self {
        Self {
            save_user_port,
            hash_password_port,
            id_generator,
        }
    }
}

impl<S: SaveUserPort, H: PasswordHasherPort, U: IdGenerator> RegistrationUseCase
    for RegistrationService<S, H, U>
{
    async fn register(
        &self,
        command: RegistrationCommand,
    ) -> DomainResult<User, RegistrationError> {
        let hashed_password = self
            .hash_password_port
            .hash(command.secret())
            .await
            .or(Err(DomainError::UseCaseError(
                RegistrationError::FailedToHashPassword,
            )))?;

        let user = User::new(
            UserId::new(self.id_generator.generate()),
            command.name().clone(),
            hashed_password.clone().value().to_owned().into(),
        );

        match self.save_user_port.save_user(&user).await {
            Ok(saved_user) => Ok(saved_user),
            Err(DomainError::ExternalServiceError(SaveUserPortError::UserAlreadyExists)) => Err(
                DomainError::UseCaseError(RegistrationError::UserAlreadyExists),
            ),
            Err(err) => Err(DomainError::UseCaseError(RegistrationError::InternalError(
                format!("{:?}", err),
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenComparisonService<T: TokenValidatorPort> {
    token_comporator: T,
}

impl<T: TokenValidatorPort> TokenComparisonService<T> {
    pub fn new(token_comporator: T) -> Self {
        Self { token_comporator }
    }
}

impl<T: TokenValidatorPort> ValidateTokenUseCase for TokenComparisonService<T> {
    async fn compare_tokens(
        &self,
        command: ValidateTokenCommand,
    ) -> DomainResult<TokenData, ValidateTokensError> {
        self.token_comporator
            .validate_token(command.first_token())
            .await
            .map_err(|err| {
                DomainError::UseCaseError(ValidateTokensError::InternalError(format!("{:?}", err)))
            })
    }
}
