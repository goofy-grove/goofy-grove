use std::sync::Arc;

use argon2::{
    Argon2, PasswordHash, PasswordHasher as ArgonPasswordHasher,
    PasswordVerifier as ArgonPasswordVerifier,
    password_hash::{Salt, SaltString, rand_core::OsRng},
};
use base64::{Engine, prelude::BASE64_STANDARD_NO_PAD};
use gg_core::domain::prelude::*;

use crate::infra::config::Config;

#[derive(Debug, Clone)]
pub struct ArgonPasswordSystem;

impl PasswordVerifierPort for ArgonPasswordSystem {
    async fn verify(
        &self,
        proposed_password: &Secret,
        confirmed_password: &Secret,
    ) -> DomainResult<(), PasswordVerifierPortError> {
        let password_hash = PasswordHash::new(confirmed_password.value()).map_err(|err| {
            DomainError::ExternalServiceError(PasswordVerifierPortError::InternalError(
                err.to_string(),
            ))
        })?;

        if Argon2::default()
            .verify_password(proposed_password.value().as_bytes(), &password_hash)
            .is_ok()
        {
            Ok(())
        } else {
            Err(DomainError::ExternalServiceError(
                PasswordVerifierPortError::PasswordNotMatch,
            ))
        }
    }
}

impl PasswordHasherPort for ArgonPasswordSystem {
    async fn hash(&self, password: &Secret) -> DomainResult<Secret, PasswordHasherPortError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        argon2
            .hash_password(password.value().as_bytes(), &salt)
            .map(|hash| Secret::new(hash.to_string()))
            .map_err(|err| {
                DomainError::ExternalServiceError(PasswordHasherPortError::InternalError(
                    err.to_string(),
                ))
            })
    }
}

#[derive(Debug, Clone)]
pub struct ArgonTokenHasher {
    config: Arc<Config>,
}

impl ArgonTokenHasher {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl TokenHasherPort for ArgonTokenHasher {
    async fn hash_token(&self, token: Token) -> DomainResult<HashedToken, TokenHasherPortError> {
        let argon2 = Argon2::default();
        let base64_salt =
            BASE64_STANDARD_NO_PAD.encode(self.config.jwt.refresh_token.salt.to_owned());
        let salt = Salt::from_b64(base64_salt.as_str()).map_err(|err| {
            DomainError::ExternalServiceError(TokenHasherPortError::InternalError(err.to_string()))
        })?;

        argon2
            .hash_password(token.value().as_bytes(), salt)
            .map(|hash| HashedToken::new(hash.to_string()))
            .map_err(|err| {
                DomainError::ExternalServiceError(TokenHasherPortError::InternalError(
                    err.to_string(),
                ))
            })
    }
}
