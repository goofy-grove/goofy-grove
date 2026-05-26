use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum FilenameValidationError {
    #[error("filename_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Filename(String);
    error: FilenameValidationError;
    sanitize: |filename: String| filename.trim().to_owned();
    validate: |filename: &str| {
        if filename.is_empty() {
            Err(FilenameValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
