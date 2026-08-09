use sea_orm::TransactionTrait;
use thiserror::Error;

use crate::{
    app::AppDeps,
    file::{
        db::file::{self, FileScope},
        services::{
            create::{self, CreateFileError, CreateFileInput},
            policy,
        },
    },
};

#[derive(Debug, Clone, Error)]
pub enum OrphanAvatarError {
    #[error("File not found")]
    FileNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum ReplaceAvatarError {
    #[error("Access denied")]
    AccessDenied,

    #[error("Policy violation")]
    PolicyViolation(#[from] policy::PolicyViolationError),

    #[error("Policy for scope not found")]
    PolicyForScopeNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<CreateFileError> for ReplaceAvatarError {
    fn from(err: CreateFileError) -> Self {
        match err {
            CreateFileError::AccessDenied => ReplaceAvatarError::AccessDenied,
            CreateFileError::PolicyViolation(violation) => {
                ReplaceAvatarError::PolicyViolation(violation)
            }
            CreateFileError::PolicyForScopeNotFound => ReplaceAvatarError::PolicyForScopeNotFound,
            CreateFileError::InternalError(message) => ReplaceAvatarError::InternalError(message),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceAvatarInput {
    pub content_type: String,
    pub original_name: String,
    pub scope: FileScope,
    pub content: Vec<u8>,
    pub current_avatar_uid: Option<String>,
}

pub async fn replace_avatar(
    deps: &AppDeps,
    input: ReplaceAvatarInput,
    user_uid: &str,
) -> Result<String, ReplaceAvatarError> {
    let ReplaceAvatarInput {
        content_type,
        original_name,
        scope,
        content,
        current_avatar_uid,
    } = input;

    let new_uid = create::create_file(
        deps,
        CreateFileInput {
            content_type,
            original_name,
            scope,
            content,
        },
        user_uid,
    )
    .await?;

    let txn = deps
        .db
        .begin()
        .await
        .map_err(|err| ReplaceAvatarError::InternalError(err.to_string()))?;

    file::activate_file(&txn, &new_uid)
        .await
        .map_err(|err| match err {
            file::UpdateFileStatusError::InternalError(message) => {
                ReplaceAvatarError::InternalError(message)
            }
        })?;

    if let Some(old_id) = current_avatar_uid
        && old_id != new_uid
    {
        match file::load_file(&txn, &old_id).await {
            Ok(_) => {
                file::orphan_file(&txn, &old_id)
                    .await
                    .map_err(|err| match err {
                        file::UpdateFileStatusError::InternalError(message) => {
                            ReplaceAvatarError::InternalError(message)
                        }
                    })?;
            }
            Err(file::LoadFileError::NotFound) => {}
            Err(file::LoadFileError::InternalError(message)) => {
                return Err(ReplaceAvatarError::InternalError(message));
            }
        }
    }

    txn.commit()
        .await
        .map_err(|err| ReplaceAvatarError::InternalError(err.to_string()))?;

    Ok(new_uid)
}

pub async fn orphan_avatar_if_present(
    deps: &AppDeps,
    avatar_uid: Option<String>,
) -> Result<(), OrphanAvatarError> {
    if let Some(file_id) = avatar_uid {
        match orphan_file_by_id(deps, &file_id).await {
            Ok(()) => {}
            Err(OrphanAvatarError::FileNotFound) => {}
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

async fn orphan_file_by_id(deps: &AppDeps, file_id: &str) -> Result<(), OrphanAvatarError> {
    let txn = deps
        .db
        .begin()
        .await
        .map_err(|err| OrphanAvatarError::InternalError(err.to_string()))?;

    file::load_file(&txn, file_id)
        .await
        .map_err(|err| match err {
            file::LoadFileError::NotFound => OrphanAvatarError::FileNotFound,
            file::LoadFileError::InternalError(message) => {
                OrphanAvatarError::InternalError(message)
            }
        })?;

    file::orphan_file(&txn, file_id)
        .await
        .map_err(|err| match err {
            file::UpdateFileStatusError::InternalError(message) => {
                OrphanAvatarError::InternalError(message)
            }
        })?;

    txn.commit()
        .await
        .map_err(|err| OrphanAvatarError::InternalError(err.to_string()))
}
