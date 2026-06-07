use sea_orm::TransactionTrait;
use thiserror::Error;

use crate::{
    app::AppDeps,
    file::{
        db::file::{self, FileScope},
        services::bind,
    },
    platform::types::PatchField,
};

#[derive(Debug, Clone, Error)]
pub enum ApplyAvatarPatchError {
    #[error("File not found")]
    FileNotFound,

    #[error("Invalid file status")]
    InvalidFileStatus,

    #[error("Invalid file scope")]
    InvalidFileScope,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum OrphanAvatarError {
    #[error("File not found")]
    FileNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn apply_avatar_uid_patch(
    deps: &AppDeps,
    current: Option<String>,
    patch: PatchField<String>,
    expected_scope: &FileScope,
) -> Result<Option<String>, ApplyAvatarPatchError> {
    match patch {
        PatchField::Unchanged => Ok(current),
        PatchField::Clear => {
            if let Some(old_id) = current {
                orphan_file_by_id(deps, &old_id)
                    .await
                    .map_err(|err| match err {
                        OrphanAvatarError::FileNotFound => ApplyAvatarPatchError::FileNotFound,
                        OrphanAvatarError::InternalError(message) => {
                            ApplyAvatarPatchError::InternalError(message)
                        }
                    })?;
            }

            Ok(None)
        }
        PatchField::Set(new_id) => {
            if current.as_ref() == Some(&new_id) {
                return Ok(current);
            }

            let txn = deps
                .db
                .begin()
                .await
                .map_err(|err| ApplyAvatarPatchError::InternalError(err.to_string()))?;

            let meta = file::load_file(&txn, &new_id)
                .await
                .map_err(|err| match err {
                    file::LoadFileError::NotFound => ApplyAvatarPatchError::FileNotFound,
                    file::LoadFileError::InternalError(message) => {
                        ApplyAvatarPatchError::InternalError(message)
                    }
                })?;

            match bind::can_bind_file_as_avatar(&meta, expected_scope) {
                bind::BindAvatarCheck::Allowed => {}
                bind::BindAvatarCheck::InvalidStatus => {
                    return Err(ApplyAvatarPatchError::InvalidFileStatus);
                }
                bind::BindAvatarCheck::InvalidScope => {
                    return Err(ApplyAvatarPatchError::InvalidFileScope);
                }
            }

            file::activate_file(&txn, &new_id)
                .await
                .map_err(|err| match err {
                    file::UpdateFileStatusError::InternalError(message) => {
                        ApplyAvatarPatchError::InternalError(message)
                    }
                })?;

            if let Some(old_id) = current
                && old_id != new_id
            {
                file::load_file(&txn, &old_id)
                    .await
                    .map_err(|err| match err {
                        file::LoadFileError::NotFound => ApplyAvatarPatchError::FileNotFound,
                        file::LoadFileError::InternalError(message) => {
                            ApplyAvatarPatchError::InternalError(message)
                        }
                    })?;

                file::orphan_file(&txn, &old_id)
                    .await
                    .map_err(|err| match err {
                        file::UpdateFileStatusError::InternalError(message) => {
                            ApplyAvatarPatchError::InternalError(message)
                        }
                    })?;
            }

            txn.commit()
                .await
                .map_err(|err| ApplyAvatarPatchError::InternalError(err.to_string()))?;

            Ok(Some(new_id))
        }
    }
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
