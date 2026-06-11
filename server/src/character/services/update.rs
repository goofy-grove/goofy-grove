use thiserror::Error;

use crate::{
    app::AppDeps, character::{
        db::character::{self, Character},
        events::types::CharacterUpdatedEvent,
    }, file::public::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch}, platform::{events::EventPublisher, types::PatchField}
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
    pub id: String,
    pub user_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_uid: PatchField<String>,
    pub exclude_participants: Vec<String>,
}

pub async fn update_character(
    deps: &AppDeps,
    input: UpdateCharacterInput,
) -> Result<Character, UpdateCharacterError> {
    let character = character::load_character(&deps.db, &input.id, &input.user_id)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::NotFound => UpdateCharacterError::NotFound,
            character::LoadCharacterError::InternalError(message) => {
                UpdateCharacterError::InternalError(message)
            }
        })?;

    if character.creator_id != input.user_id {
        return Err(UpdateCharacterError::AccessDenied);
    }

    let expected_scope = FileScope::CharacterAvatar {
        user_id: input.user_id.clone(),
        character_id: character.uid.clone(),
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
        ApplyAvatarPatchError::InternalError(message) => UpdateCharacterError::InternalError(message),
    })?;

    let updated = Character {
        uid: character.uid,
        creator_id: character.creator_id,
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
