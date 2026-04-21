use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum PersonaIdValidationError {
    #[error("persona_name_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PersonaId(String);
    error: PersonaIdValidationError;
    sanitize: |id: String| id.trim().to_owned();
    validate: |id: &str| {
        if id.is_empty() {
            Err(PersonaIdValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
