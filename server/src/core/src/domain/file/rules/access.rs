use thiserror::Error;

use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FileAccessError {
    #[error("Access denied")]
    AccessDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileCreateAccessContext {
    pub persona_owned_by_actor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetaAccessContext {
    pub persona_owned_by_actor: bool,
}

fn scope_owner(scope: &FileScope) -> &UserId {
    match scope {
        FileScope::PersonaAvatar { user_id, .. } => user_id,
        FileScope::UserAvatar { user_id } => user_id,
    }
}

pub fn can_create_file(
    actor: &UserId,
    scope: &FileScope,
    ctx: &FileCreateAccessContext,
) -> Result<(), FileAccessError> {
    if actor != scope_owner(scope) {
        return Err(FileAccessError::AccessDenied);
    }

    if matches!(scope, FileScope::PersonaAvatar { .. }) && !ctx.persona_owned_by_actor {
        return Err(FileAccessError::AccessDenied);
    }

    Ok(())
}

pub fn can_read_file(
    actor: &UserId,
    meta: &FileMeta,
    ctx: &FileMetaAccessContext,
) -> Result<(), FileAccessError> {
    can_access_file_meta(actor, meta, ctx)
}

pub fn can_delete_file(
    actor: &UserId,
    meta: &FileMeta,
    ctx: &FileMetaAccessContext,
) -> Result<(), FileAccessError> {
    can_access_file_meta(actor, meta, ctx)
}

fn can_access_file_meta(
    actor: &UserId,
    meta: &FileMeta,
    ctx: &FileMetaAccessContext,
) -> Result<(), FileAccessError> {
    if actor != scope_owner(&meta.scope) {
        return Err(FileAccessError::AccessDenied);
    }

    if matches!(meta.scope, FileScope::PersonaAvatar { .. }) && !ctx.persona_owned_by_actor {
        return Err(FileAccessError::AccessDenied);
    }

    Ok(())
}
