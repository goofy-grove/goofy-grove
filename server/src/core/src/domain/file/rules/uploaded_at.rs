use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum UploadedAtValidationError {
    #[error("uploaded_at_non_positive")]
    NonPositive,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UploadedAt(i64);
    error: UploadedAtValidationError;
    validate: |uploaded_at: &i64| {
        if !uploaded_at.is_positive() {
            Err(UploadedAtValidationError::NonPositive)
        } else {
            Ok(())
        }
    };
);
