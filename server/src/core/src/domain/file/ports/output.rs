use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum SaveFileToStoragePortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait SaveFileToStoragePort {
    fn save_file_to_storage(
        &self,
        meta: &FileMeta,
        content: FileContent,
    ) -> impl Future<Output = Result<(), SaveFileToStoragePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum SaveFilePortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait SaveFilePort {
    fn save_file(&self, meta: FileMeta) -> impl Future<Output = Result<FileId, SaveFilePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteFilePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait DeleteFilePort {
    fn delete_file(&self, id: FileId) -> impl Future<Output = Result<(), DeleteFilePortError>>;
}
