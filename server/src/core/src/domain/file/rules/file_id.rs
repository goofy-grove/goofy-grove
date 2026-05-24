use thiserror::Error;

use crate::impl_new_type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum FileIdValidationError {
    #[error("file_id_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FileId(String);
    error: FileIdValidationError;
    sanitize: |id: String| id.trim().to_owned();
    validate: |id: &str| {
        if id.is_empty() {
            Err(FileIdValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
