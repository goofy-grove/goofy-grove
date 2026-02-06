use argon2::{Argon2, PasswordHash, PasswordVerifier as ArgonPasswordVerifier};
use domain::prelude::*;

#[derive(Debug, Clone)]
pub struct PasswordVerifier;

impl PasswordVerifierPort for PasswordVerifier {
    async fn verify(
        &self,
        password: &UserPassword,
        hashed_password: &UserPassword,
    ) -> DomainResult<(), PasswordVerifierPortError> {
        let password_hash = PasswordHash::new(hashed_password.value()).map_err(|err| {
            DomainError::ExternalServiceError(PasswordVerifierPortError::InternalError(
                err.to_string(),
            ))
        })?;

        if Argon2::default()
            .verify_password(password.value().as_bytes(), &password_hash)
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
