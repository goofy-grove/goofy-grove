use std::path::PathBuf;
use std::sync::Arc;

use gg_core::domain::prelude::*;
use tokio::fs;

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
        self.base_dir.join(meta.filename.inner())
    }
}

impl SaveFileToStoragePort for LocalFileStorage {
    async fn save_file_to_storage(
        &self,
        meta: &FileMeta,
        content: FileContent,
    ) -> Result<(), SaveFileToStoragePortError> {
        fs::create_dir_all(&*self.base_dir)
            .await
            .map_err(|err| SaveFileToStoragePortError::InternalError(err.to_string()))?;

        fs::write(self.path_for(meta), content.into_inner())
            .await
            .map_err(|err| SaveFileToStoragePortError::InternalError(err.to_string()))
    }
}

impl LoadFileFromStoragePort for LocalFileStorage {
    async fn load_file_from_storage(
        &self,
        meta: &FileMeta,
    ) -> Result<FileContent, LoadFileFromStoragePortError> {
        let path = self.path_for(meta);
        if !path.exists() {
            return Err(LoadFileFromStoragePortError::FileNotFound);
        }

        let bytes = fs::read(path)
            .await
            .map_err(|err| LoadFileFromStoragePortError::InternalError(err.to_string()))?;

        Ok(FileContent::new(bytes))
    }
}

impl DeleteFileFromStoragePort for LocalFileStorage {
    async fn delete_file_from_storage(
        &self,
        meta: &FileMeta,
    ) -> Result<(), DeleteFileFromStoragePortError> {
        let path = self.path_for(meta);
        if !path.exists() {
            return Err(DeleteFileFromStoragePortError::FileNotFound);
        }

        fs::remove_file(path)
            .await
            .map_err(|err| DeleteFileFromStoragePortError::InternalError(err.to_string()))
    }
}
