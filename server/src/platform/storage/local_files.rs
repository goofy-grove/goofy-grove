use std::path::PathBuf;
use std::sync::Arc;

use thiserror::Error;
use tokio::fs;

use crate::file::db::file::FileMeta;

#[derive(Debug, Clone, Error)]
pub enum LoadStorageError {
    #[error("File not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum SaveStorageError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum DeleteStorageError {
    #[error("File not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct LocalFileStorage {
    base_dir: Arc<PathBuf>,
}

impl LocalFileStorage {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: Arc::new(base_dir.into()),
        }
    }

    fn path_for(&self, meta: &FileMeta) -> PathBuf {
        self.base_dir.join(&meta.filename)
    }

    pub async fn save(&self, meta: &FileMeta, content: &[u8]) -> Result<(), SaveStorageError> {
        fs::create_dir_all(&*self.base_dir)
            .await
            .map_err(|err| SaveStorageError::InternalError(err.to_string()))?;

        fs::write(self.path_for(meta), content)
            .await
            .map_err(|err| SaveStorageError::InternalError(err.to_string()))
    }

    pub async fn load(&self, meta: &FileMeta) -> Result<Vec<u8>, LoadStorageError> {
        let path = self.path_for(meta);
        if !path.exists() {
            return Err(LoadStorageError::NotFound);
        }

        fs::read(path)
            .await
            .map_err(|err| LoadStorageError::InternalError(err.to_string()))
    }

    pub async fn delete(&self, meta: &FileMeta) -> Result<(), DeleteStorageError> {
        let path = self.path_for(meta);
        if !path.exists() {
            return Err(DeleteStorageError::NotFound);
        }

        fs::remove_file(path)
            .await
            .map_err(|err| DeleteStorageError::InternalError(err.to_string()))
    }
}
