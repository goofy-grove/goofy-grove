use thiserror::Error;

use crate::impl_new_type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum TokenValidationError {
    #[error("token_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Token(String);
    error: TokenValidationError;
    sanitize: |token: String| token.trim().to_owned();
    validate: |token: &str| {
        if token.is_empty() {
            Err(TokenValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
