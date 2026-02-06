use domain::prelude::*;

#[derive(Debug, Clone)]
pub struct PasswordVerifier;

impl PasswordVerifierPort for PasswordVerifier {
    async fn verify(
        &self,
        password: &UserPassword,
        hashed_password: &UserPassword,
    ) -> DomainResult<(), PasswordVerifierPortError> {
        if password == hashed_password {
            Ok(())
        } else {
            Err(DomainError::ExternalServiceError(
                PasswordVerifierPortError::PasswordNotMatch,
            ))
        }
    }
}
