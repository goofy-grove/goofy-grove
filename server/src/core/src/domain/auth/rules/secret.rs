use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum SecretValidationError {
    #[error("secret_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Secret(String);
    error: SecretValidationError;
    sanitize: |secret: String| secret.trim().to_owned();
    validate: |secret: &str| {
        if secret.is_empty() {
            Err(SecretValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
