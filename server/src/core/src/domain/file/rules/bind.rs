use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FileBindError {
    #[error("File not found")]
    FileNotFound,

    #[error("Invalid file status")]
    InvalidStatus,

    #[error("Invalid file scope")]
    InvalidScope,

    #[error("Access denied")]
    AccessDenied,
}

pub fn can_bind_file_as_avatar(
    meta: &FileMeta,
    expected_scope: &FileScope,
) -> Result<(), FileBindError> {
    if meta.status != FileStatus::Created {
        return Err(FileBindError::InvalidStatus);
    }

    if &meta.scope != expected_scope {
        return Err(FileBindError::InvalidScope);
    }

    Ok(())
}

pub fn can_serve_file(meta: &FileMeta) -> Result<(), FileBindError> {
    match meta.status {
        FileStatus::Created | FileStatus::Activated => Ok(()),
        FileStatus::Orphaned => Err(FileBindError::FileNotFound),
    }
}
