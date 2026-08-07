use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{
        db::character::{self, Character},
        events::types::CharacterUpdatedEvent,
    },
    file::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch},
    platform::{events::EventPublisher, types::PatchField},
};

#[derive(Debug, Clone, Error)]
pub enum UpdateCharacterError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Access denied")]
    AccessDenied,

    #[error("File not found")]
    FileNotFound,

    #[error("Invalid file status")]
    InvalidFileStatus,

    #[error("Invalid file scope")]
    InvalidFileScope,
}

#[derive(Debug, Clone)]
pub struct UpdateCharacterInput {
    pub character_uid: String,
    pub user_uid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_uid: PatchField<String>,
    pub exclude_participants: Vec<String>,
}

pub async fn update_character(
    deps: &AppDeps,
    input: UpdateCharacterInput,
) -> Result<Character, UpdateCharacterError> {
    let character = character::load_character(&deps.db, &input.character_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::NotFound => UpdateCharacterError::NotFound,
            character::LoadCharacterError::InternalError(message) => {
                UpdateCharacterError::InternalError(message)
            }
        })?;

    if character.creator_uid != input.user_uid {
        return Err(UpdateCharacterError::AccessDenied);
    }

    let expected_scope = FileScope::CharacterAvatar {
        user_uid: input.user_uid.clone(),
        character_uid: character.uid.clone(),
    };

    let next_avatar_uid = apply_avatar_uid_patch(
        deps,
        character.avatar_uid.clone(),
        input.avatar_uid,
        &expected_scope,
    )
    .await
    .map_err(|err| match err {
        ApplyAvatarPatchError::FileNotFound => UpdateCharacterError::FileNotFound,
        ApplyAvatarPatchError::InvalidFileStatus => UpdateCharacterError::InvalidFileStatus,
        ApplyAvatarPatchError::InvalidFileScope => UpdateCharacterError::InvalidFileScope,
        ApplyAvatarPatchError::InternalError(message) => {
            UpdateCharacterError::InternalError(message)
        }
    })?;

    let updated = Character {
        uid: character.uid,
        creator_uid: character.creator_uid,
        name: input.name.unwrap_or(character.name),
        description: input.description.unwrap_or(character.description),
        avatar_uid: next_avatar_uid,
    };

    let saved = character::save_character(&deps.db, updated)
        .await
        .map_err(|err| UpdateCharacterError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(CharacterUpdatedEvent {
            character: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
