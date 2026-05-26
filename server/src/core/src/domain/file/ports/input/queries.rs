use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum GetFileQueryError {
    #[error("File not found")]
    FileNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Access denied")]
    AccessDenied,
}

pub trait GetFileQuery {
    fn get_file(
        &self,
        file_id: FileId,
        user_id: UserId,
    ) -> impl Future<Output = Result<FileContent, GetFileQueryError>>;
}
