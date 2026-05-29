use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum FileSizeValidationError {
    #[error("file_size_zero")]
    ZeroSize,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FileSize(usize);
    error: FileSizeValidationError;
    validate: |size: &usize| {
        if *size == 0 {
            Err(FileSizeValidationError::ZeroSize)
        } else {
            Ok(())
        }
    };
);
