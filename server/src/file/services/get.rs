use thiserror::Error;

use crate::{
    app::AppDeps,
    file::{
        db::file,
        services::{access, bind},
    },
    platform::storage::{LoadStorageError, LocalFileStorage},
};

#[derive(Debug, Clone, Error)]
pub enum GetFileError {
    #[error("File not found")]
    NotFound,

    #[error("Access denied")]
    AccessDenied,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn get_file(
    deps: &AppDeps,
    file_uid: &str,
    user_uid: &str,
) -> Result<Vec<u8>, GetFileError> {
    let meta = file::load_file(&deps.db, file_uid)
        .await
        .map_err(|err| match err {
            file::LoadFileError::NotFound => GetFileError::NotFound,
            file::LoadFileError::InternalError(message) => GetFileError::InternalError(message),
        })?;

    access::can_access_file_meta(deps, user_uid, &meta)
        .await
        .map_err(|_| GetFileError::AccessDenied)?;

    if !bind::can_serve_file(&meta) {
        return Err(GetFileError::NotFound);
    }

    let storage = LocalFileStorage::new(deps.config.storage.files_dir.clone());

    storage.load(&meta).await.map_err(|err| match err {
        LoadStorageError::NotFound => GetFileError::NotFound,
        LoadStorageError::InternalError(message) => GetFileError::InternalError(message),
    })
}
