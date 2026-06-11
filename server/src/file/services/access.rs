use thiserror::Error;

use crate::{
    app::AppDeps,
    file::db::file::{FileMeta, FileScope},
    persona,
    character,
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FileAccessError {
    #[error("Access denied")]
    AccessDenied,
}

fn scope_owner(scope: &FileScope) -> &str {
    match scope {
        FileScope::PersonaAvatar { user_id, .. } => user_id,
        FileScope::UserAvatar { user_id } => user_id,
        FileScope::CharacterAvatar { user_id, .. } => user_id,
    }
}

pub async fn can_create_file(
    deps: &AppDeps,
    actor_id: &str,
    scope: &FileScope,
) -> Result<(), FileAccessError> {
    if actor_id != scope_owner(scope) {
        return Err(FileAccessError::AccessDenied);
    }

    match scope {
        FileScope::UserAvatar { .. } => Ok(()),
        FileScope::PersonaAvatar {
            persona_id,
            user_id,
        } => {
            if persona::public::is_owner(deps, persona_id, user_id).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        }
        FileScope::CharacterAvatar { user_id, character_id } => {
            if character::public::is_owner(deps, character_id, user_id).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        },
    }
}

pub async fn can_access_file_meta(
    deps: &AppDeps,
    actor_id: &str,
    meta: &FileMeta,
) -> Result<(), FileAccessError> {
    if actor_id != scope_owner(&meta.scope) {
        return Err(FileAccessError::AccessDenied);
    }

    match &meta.scope {
        FileScope::UserAvatar { .. } => Ok(()),
        FileScope::PersonaAvatar {
            persona_id,
            user_id,
        } => {
            if persona::public::is_owner(deps, persona_id, user_id).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        }
        FileScope::CharacterAvatar { user_id, character_id } => {
            if character::public::is_owner(deps, character_id, user_id).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        },
    }
}
