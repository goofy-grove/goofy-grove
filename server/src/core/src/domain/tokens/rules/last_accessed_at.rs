use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum LastAccessedAtValidationError {
    #[error("last_accessed_at_non_positive")]
    NonPositive,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct LastAccessedAt(i64);
    error: LastAccessedAtValidationError;
    validate: |last_accessed_at: &i64| {
        if !last_accessed_at.is_positive() {
            Err(LastAccessedAtValidationError::NonPositive)
        } else {
            Ok(())
        }
    };
);
