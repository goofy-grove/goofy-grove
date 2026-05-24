use thiserror::Error;

use crate::impl_new_type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum FileOriginalNameValidationError {
    #[error("file_original_name_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FileOriginalName(String);
    error: FileOriginalNameValidationError;
    sanitize: |original_name: String| original_name.trim().to_owned();
    validate: |original_name: &str| {
        if original_name.is_empty() {
            Err(FileOriginalNameValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
