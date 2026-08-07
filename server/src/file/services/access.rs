use thiserror::Error;

use crate::{
    app::AppDeps,
    character,
    file::db::file::{FileMeta, FileScope},
    persona,
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FileAccessError {
    #[error("Access denied")]
    AccessDenied,
}

fn scope_owner(scope: &FileScope) -> &str {
    match scope {
        FileScope::PersonaAvatar { user_uid, .. } => user_uid,
        FileScope::UserAvatar { user_uid } => user_uid,
        FileScope::CharacterAvatar { user_uid, .. } => user_uid,
    }
}

pub async fn can_create_file(
    deps: &AppDeps,
    actor_uid: &str,
    scope: &FileScope,
) -> Result<(), FileAccessError> {
    if actor_uid != scope_owner(scope) {
        return Err(FileAccessError::AccessDenied);
    }

    match scope {
        FileScope::UserAvatar { .. } => Ok(()),
        FileScope::PersonaAvatar {
            persona_uid,
            user_uid,
        } => {
            if persona::is_owner(deps, persona_uid, user_uid).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        }
        FileScope::CharacterAvatar {
            user_uid,
            character_uid,
        } => {
            if character::is_owner(deps, character_uid, user_uid).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        }
    }
}

pub async fn can_access_file_meta(
    deps: &AppDeps,
    actor_uid: &str,
    meta: &FileMeta,
) -> Result<(), FileAccessError> {
    if actor_uid != scope_owner(&meta.scope) {
        return Err(FileAccessError::AccessDenied);
    }

    match &meta.scope {
        FileScope::UserAvatar { .. } => Ok(()),
        FileScope::PersonaAvatar {
            persona_uid,
            user_uid,
        } => {
            if persona::is_owner(deps, persona_uid, user_uid).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        }
        FileScope::CharacterAvatar {
            user_uid,
            character_uid,
        } => {
            if character::is_owner(deps, character_uid, user_uid).await {
                Ok(())
            } else {
                Err(FileAccessError::AccessDenied)
            }
        }
    }
}
