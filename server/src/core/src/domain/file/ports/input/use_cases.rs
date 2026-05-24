use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum CreateFileError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait CreateFileUseCase {
    fn create_file(
        &self,
        command: CreateFileCommand,
    ) -> impl Future<Output = Result<FileId, CreateFileError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteFileError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait DeleteFileUseCase {
    fn delete_file(
        &self,
        command: DeleteFileCommand,
    ) -> impl Future<Output = Result<(), DeleteFileError>>;
}

#[derive(Debug, Clone, Error)]
pub enum ReplaceFileError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait ReplaceFileUseCase {
    fn replace_file(
        &self,
        command: ReplaceFileCommand,
    ) -> impl Future<Output = Result<FileMeta, ReplaceFileError>>;
}
