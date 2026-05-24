use thiserror::Error;

use crate::domain::file::rules::FileId;

#[derive(Debug, Clone, Error)]
pub enum GetFileQueryError {
    #[error("File not found")]
    FileNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait GetFileQuery {
    fn get_file(&self, file_id: FileId) -> impl Future<Output = Result<Vec<u8>, GetFileQueryError>>;
}
