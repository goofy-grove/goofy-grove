#[cfg(test)]
mod tests;

use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum AvatarBindingError {
    #[error("File not found")]
    FileNotFound,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn apply_avatar_uid_patch<L, A, O>(
    load_file_port: &L,
    activate_file_port: &A,
    orphan_file_port: &O,
    current: Option<FileId>,
    patch: PatchField<FileId>,
    expected_scope: &FileScope,
) -> Result<Option<FileId>, AvatarBindingError>
where
    L: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
{
    match patch {
        PatchField::Unchanged => Ok(current),
        PatchField::Clear => {
            if let Some(old_id) = current {
                orphan_existing_file(load_file_port, orphan_file_port, old_id).await?;
            }
            Ok(None)
        }
        PatchField::Set(new_id) => {
            if current.as_ref() == Some(&new_id) {
                return Ok(current);
            }

            let meta = load_file_port
                .load_file(new_id.clone())
                .await
                .map_err(|err| match err {
                    LoadFilePortError::FileNotFound => AvatarBindingError::FileNotFound,
                    LoadFilePortError::InternalError(message) => {
                        AvatarBindingError::InternalError(message)
                    }
                })?;

            can_bind_file_as_avatar(&meta, expected_scope).map_err(|err| match err {
                FileBindError::FileNotFound => AvatarBindingError::FileNotFound,
                FileBindError::InvalidStatus | FileBindError::InvalidScope => {
                    AvatarBindingError::ValidationError(err.to_string())
                }
                FileBindError::AccessDenied => AvatarBindingError::ValidationError(err.to_string()),
            })?;

            activate_file_port
                .activate_file(&meta)
                .await
                .map_err(|err| match err {
                    ActivateFilePortError::FileNotFound => AvatarBindingError::FileNotFound,
                    ActivateFilePortError::InternalError(message) => {
                        AvatarBindingError::InternalError(message)
                    }
                })?;

            if let Some(old_id) = current
                && old_id != new_id
            {
                orphan_existing_file(load_file_port, orphan_file_port, old_id).await?;
            }

            Ok(Some(new_id))
        }
    }
}

pub async fn orphan_avatar_if_present<L, O>(
    load_file_port: &L,
    orphan_file_port: &O,
    avatar_uid: Option<FileId>,
) -> Result<(), AvatarBindingError>
where
    L: LoadFilePort,
    O: OrphanFilePort,
{
    if let Some(file_id) = avatar_uid {
        orphan_existing_file(load_file_port, orphan_file_port, file_id).await?;
    }
    Ok(())
}

async fn orphan_existing_file<L, O>(
    load_file_port: &L,
    orphan_file_port: &O,
    file_id: FileId,
) -> Result<(), AvatarBindingError>
where
    L: LoadFilePort,
    O: OrphanFilePort,
{
    let meta = load_file_port
        .load_file(file_id)
        .await
        .map_err(|err| match err {
            LoadFilePortError::FileNotFound => AvatarBindingError::FileNotFound,
            LoadFilePortError::InternalError(message) => AvatarBindingError::InternalError(message),
        })?;

    orphan_file_port
        .orphan_file(&meta)
        .await
        .map_err(|err| match err {
            OrphanFilePortError::FileNotFound => AvatarBindingError::FileNotFound,
            OrphanFilePortError::InternalError(message) => {
                AvatarBindingError::InternalError(message)
            }
        })
}
