use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum CreateFileError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Access denied")]
    AccessDenied,

    #[error("Policy violation: {0}")]
    PolicyViolation(FilePolicyViolationError),

    #[error("Policy not found for scope")]
    PolicyForScopeNotFound,
}

pub trait CreateFileUseCase {
    fn create_file(
        &self,
        command: CreateFileCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<FileId, CreateFileError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteFileError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,

    #[error("Access denied")]
    AccessDenied,
}

pub trait DeleteFileUseCase {
    fn delete_file(
        &self,
        command: DeleteFileCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), DeleteFileError>>;
}
