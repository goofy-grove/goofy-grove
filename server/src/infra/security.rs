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
