use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct UserAuthenticationService<L: LoadUserByNamePort, C: PasswordVerifierPort> {
    load_user_port: L,
    compare_password_port: C,
}

impl<L: LoadUserByNamePort, C: PasswordVerifierPort> UserAuthenticationService<L, C> {
    pub fn new(load_user_port: L, compare_password_port: C) -> Self {
        Self {
            load_user_port,
            compare_password_port,
        }
    }
}

impl<L: LoadUserByNamePort, C: PasswordVerifierPort> AuthenticationUseCase
    for UserAuthenticationService<L, C>
{
    async fn authenticate(
        &self,
        command: AuthenticationCommand,
    ) -> Result<User, AuthenticationError> {
        let AuthenticationCommand { name, secret } = command;

        let user = self
            .load_user_port
            .load_user_by_name(&name)
            .await
            .or(Err(AuthenticationError::UserNotFound))?;

        self.compare_password_port
            .verify(
                &secret,
                // NOTE: this unwrap is safe, because user.password has same checks and already checked
                &user.password.inner().to_owned().try_into().unwrap(),
            )
            .await
            .map(|_| user)
            .or(Err(AuthenticationError::InvalidCredentials))
    }
}

#[derive(Debug, Clone)]
pub struct RegistrationPrerequisites<S, H, U> {
    pub save_user_port: S,
    pub hash_password_port: H,
    pub id_generator: U,
}

#[derive(Debug, Clone)]
pub struct RegistrationService<S: SaveUserPort, H: PasswordHasherPort, U: IdGenerator> {
    save_user_port: S,
    hash_password_port: H,
    id_generator: U,
}

impl<S: SaveUserPort, H: PasswordHasherPort, U: IdGenerator> RegistrationService<S, H, U> {
    pub fn new(prerequisites: RegistrationPrerequisites<S, H, U>) -> Self {
        let RegistrationPrerequisites {
            save_user_port,
            hash_password_port,
            id_generator,
        } = prerequisites;

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
    async fn register(&self, command: RegistrationCommand) -> Result<User, RegistrationError> {
        let RegistrationCommand { name, secret } = command;

        let hashed_password = self
            .hash_password_port
            .hash(&secret)
            .await
            .or(Err(RegistrationError::FailedToHashPassword))?;

        let user = User {
            uid: UserId::try_new(self.id_generator.generate())
                .map_err(|err| RegistrationError::ValidationError(format!("{err}")))?,
            name,
            // NOTE: this unwrap is safe, because hashed_password has same checks and already checked
            password: hashed_password.into_inner().try_into().unwrap(),
            avatar_uid: None,
        };

        match self.save_user_port.save_user(user).await {
            Ok(saved_user) => Ok(saved_user),
            Err(SaveUserPortError::UserAlreadyExists) => Err(RegistrationError::UserAlreadyExists),
            Err(err) => Err(RegistrationError::InternalError(format!("{:?}", err))),
        }
    }
}
