use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum HashedTokenValidationError {
    #[error("hashed_token_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct HashedToken(String);
    error: HashedTokenValidationError;
    sanitize: |token: String| token.trim().to_owned();
    validate: |token: &str| {
        if token.is_empty() {
            Err(HashedTokenValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
