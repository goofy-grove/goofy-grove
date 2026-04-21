use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum TokenIdValidationError {
    #[error("token_id_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TokenId(String);
    error: TokenIdValidationError;
    sanitize: |id: String| id.trim().to_owned();
    validate: |id: &str| {
        if id.is_empty() {
            Err(TokenIdValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
