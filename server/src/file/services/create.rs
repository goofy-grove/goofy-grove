use chrono::Utc;
use thiserror::Error;

use crate::{
    app::AppDeps,
    file::{
        db::file::{self, FileMeta, FileScope, FileStatus},
        services::{access, filename, policy},
    },
    platform::{
        storage::{LocalFileStorage, SaveStorageError},
        util,
    },
};

#[derive(Debug, Clone)]
pub struct CreateFileInput {
    pub content_type: String,
    pub original_name: String,
    pub scope: FileScope,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Error)]
pub enum CreateFileError {
    #[error("Access denied")]
    AccessDenied,

    #[error("Policy violation")]
    PolicyViolation(#[from] policy::PolicyViolationError),

    #[error("Policy for scope not found")]
    PolicyForScopeNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn create_file(
    deps: &AppDeps,
    input: CreateFileInput,
    user_id: &str,
) -> Result<String, CreateFileError> {
    access::can_create_file(deps, user_id, &input.scope)
        .await
        .map_err(|_| CreateFileError::AccessDenied)?;

    let policy = deps
        .config
        .policies
        .files
        .policy_for_scope(&input.scope)
        .ok_or(CreateFileError::PolicyForScopeNotFound)?;

    policy::assert_file_matches_policy(input.content.len(), &input.content_type, policy)
        .map_err(CreateFileError::PolicyViolation)?;

    let uid = util::id_generator::generate_id("file");
    let filename = filename::resolve_filename(&uid, &input.original_name);
    let meta = FileMeta {
        uid: uid.clone(),
        filename,
        uploaded_by: user_id.to_string(),
        scope: input.scope,
        uploaded_at: Utc::now(),
        status: FileStatus::Created,
        original_name: input.original_name,
        content_type: input.content_type,
        size: input.content.len(),
    };

    let storage = LocalFileStorage::new(deps.config.storage.files_dir.clone());

    storage
        .save(&meta, &input.content)
        .await
        .map_err(|err| match err {
            SaveStorageError::InternalError(message) => CreateFileError::InternalError(message),
        })?;

    match file::save_file(&deps.db, &meta).await {
        Ok(()) => Ok(uid),
        Err(err) => {
            let _ = storage.delete(&meta).await;

            Err(CreateFileError::InternalError(err.to_string()))
        }
    }
}
