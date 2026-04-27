use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum CharacterNameValidationError {
    #[error("character_name_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct CharacterName(String);
    error: CharacterNameValidationError;
    sanitize: |name: String| name.trim().to_owned();
    validate: |name: &str| {
        if name.is_empty() {
            Err(CharacterNameValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
