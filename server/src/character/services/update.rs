use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{
        db::character::{self, Character},
        events::types::CharacterUpdatedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum UpdateCharacterError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct UpdateCharacterInput {
    pub character_uid: String,
    pub user_uid: String,
    pub name: Option<String>,
    pub description: Option<String>,
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

    let updated = Character {
        uid: character.uid,
        creator_uid: character.creator_uid,
        name: input.name.unwrap_or(character.name),
        description: input.description.unwrap_or(character.description),
        avatar_uid: character.avatar_uid,
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
