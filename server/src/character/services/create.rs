use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{
        db::character::{self, Character},
        events::types::CharacterCreatedEvent,
    },
    file::public::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch},
    platform::{events::EventPublisher, types::PatchField, util},
};

#[derive(Debug, Clone, Error)]
pub enum CreateCharacterError {
    #[error("File not found")]
    FileNotFound,

    #[error("Invalid file status")]
    InvalidFileStatus,

    #[error("Invalid file scope")]
    InvalidFileScope,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct CreateCharacterInput {
    pub name: String,
    pub description: String,
    pub creator_uid: String,
    pub avatar_uid: Option<String>,
    pub exclude_participants: Vec<String>,
}

pub async fn create_character(
    deps: &AppDeps,
    input: CreateCharacterInput,
) -> Result<Character, CreateCharacterError> {
    let uid = util::uid_generator::generate_uid("character");

    let avatar_uid = if let Some(file_uid) = input.avatar_uid {
        let scope = FileScope::CharacterAvatar {
            user_uid: input.creator_uid.clone(),
            character_uid: uid.clone(),
        };

        apply_avatar_uid_patch(deps, None, PatchField::Set(file_uid), &scope)
            .await
            .map_err(|err| match err {
                ApplyAvatarPatchError::FileNotFound => CreateCharacterError::FileNotFound,
                ApplyAvatarPatchError::InvalidFileStatus => CreateCharacterError::InvalidFileStatus,
                ApplyAvatarPatchError::InvalidFileScope => CreateCharacterError::InvalidFileScope,
                ApplyAvatarPatchError::InternalError(message) => {
                    CreateCharacterError::InternalError(message)
                }
            })?
    } else {
        None
    };

    let character = Character {
        uid,
        creator_uid: input.creator_uid,
        name: input.name,
        description: input.description,
        avatar_uid,
    };

    let saved = character::save_character(&deps.db, character)
        .await
        .map_err(|err| CreateCharacterError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(CharacterCreatedEvent {
            character: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
