use argon2::{
    Argon2, PasswordHash, PasswordHasher as ArgonPasswordHasher,
    PasswordVerifier as ArgonPasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use gg_core::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct ArgonPasswordSystem;

impl PasswordVerifierPort for ArgonPasswordSystem {
    async fn verify(
        &self,
        proposed_password: &Secret,
        confirmed_password: &Secret,
    ) -> Result<(), PasswordVerifierPortError> {
        let password_hash = PasswordHash::new(confirmed_password.inner())
            .map_err(|err| PasswordVerifierPortError::InternalError(err.to_string()))?;

        if Argon2::default()
            .verify_password(proposed_password.inner().as_bytes(), &password_hash)
            .is_ok()
        {
            Ok(())
        } else {
            Err(PasswordVerifierPortError::PasswordNotMatch)
        }
    }
}

impl PasswordHasherPort for ArgonPasswordSystem {
    async fn hash(&self, password: &Secret) -> Result<Secret, PasswordHasherPortError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        argon2
            .hash_password(password.inner().as_bytes(), &salt)
            .map_err(|err| PasswordHasherPortError::InternalError(err.to_string()))
            .and_then(|hash| {
                Secret::try_new(hash.to_string())
                    .map_err(|err| PasswordHasherPortError::InternalError(err.to_string()))
            })
    }
}

#[derive(Debug, Clone)]
pub struct ArgonTokenHasher;

impl TokenHasherPort for ArgonTokenHasher {
    async fn hash_token(&self, token: Token) -> Result<HashedToken, TokenHasherPortError> {
        HashedToken::try_new(blake3::hash(token.inner().as_bytes()).to_hex().to_string())
            .map_err(|err| TokenHasherPortError::InternalError(err.to_string()))
    }
}
