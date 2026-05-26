use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum FileContentTypeValidationError {
    #[error("file_content_type_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FileContentType(String);
    error: FileContentTypeValidationError;
    sanitize: |content_type: String| content_type.trim().to_lowercase().split(';').next().map(str::trim).unwrap_or_default().to_owned();
    validate: |content_type: &str| {
        if content_type.is_empty() {
            Err(FileContentTypeValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
