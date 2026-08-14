use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{
        db::character::{self, Character},
        events::types::CharacterUpdatedEvent,
    },
    file::{
        FileScope, OrphanAvatarError, ReplaceAvatarError, ReplaceAvatarInput,
        orphan_avatar_if_present, replace_avatar,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum SetCharacterAvatarError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    ReplaceAvatar(#[from] ReplaceAvatarError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum ClearCharacterAvatarError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct SetCharacterAvatarInput {
    pub character_uid: String,
    pub user_uid: String,
    pub content_type: String,
    pub original_name: String,
    pub content: Vec<u8>,
    pub exclude_participants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClearCharacterAvatarInput {
    pub character_uid: String,
    pub user_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn set_character_avatar(
    deps: &AppDeps,
    input: SetCharacterAvatarInput,
) -> Result<Character, SetCharacterAvatarError> {
    let character = character::load_character(&deps.db, &input.character_uid)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::NotFound => SetCharacterAvatarError::NotFound,
            character::LoadCharacterError::InternalError(message) => {
                SetCharacterAvatarError::InternalError(message)
            }
        })?;

    if character.creator_uid != input.user_uid {
        return Err(SetCharacterAvatarError::NotFound);
    }

    let new_avatar_uid = replace_avatar(
        deps,
        ReplaceAvatarInput {
            content_type: input.content_type,
            original_name: input.original_name,
            scope: FileScope::CharacterAvatar {
                user_uid: input.user_uid.clone(),
                character_uid: character.uid.clone(),
            },
            content: input.content,
            current_avatar_uid: character.avatar_uid.clone(),
        },
        &input.user_uid,
    )
    .await?;

    let updated = Character {
        uid: character.uid,
        creator_uid: character.creator_uid,
        name: character.name,
        description: character.description,
        avatar_uid: Some(new_avatar_uid),
    };

    let saved = character::save_character(&deps.db, updated)
        .await
        .map_err(|err| SetCharacterAvatarError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(CharacterUpdatedEvent {
            character: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}

pub async fn clear_character_avatar(
    deps: &AppDeps,
    input: ClearCharacterAvatarInput,
) -> Result<Character, ClearCharacterAvatarError> {
    let character = character::load_character(&deps.db, &input.character_uid)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::NotFound => ClearCharacterAvatarError::NotFound,
            character::LoadCharacterError::InternalError(message) => {
                ClearCharacterAvatarError::InternalError(message)
            }
        })?;

    if character.creator_uid != input.user_uid {
        return Err(ClearCharacterAvatarError::NotFound);
    }

    orphan_avatar_if_present(deps, character.avatar_uid.clone())
        .await
        .map_err(|err| match err {
            OrphanAvatarError::FileNotFound | OrphanAvatarError::InternalError(_) => {
                ClearCharacterAvatarError::InternalError(err.to_string())
            }
        })?;

    let updated = Character {
        uid: character.uid,
        creator_uid: character.creator_uid,
        name: character.name,
        description: character.description,
        avatar_uid: None,
    };

    let saved = character::save_character(&deps.db, updated)
        .await
        .map_err(|err| ClearCharacterAvatarError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(CharacterUpdatedEvent {
            character: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
