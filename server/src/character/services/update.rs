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
    pub id: String,
    pub user_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
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

    let updated = Character {
        uid: character.uid,
        creator_id: character.creator_id,
        name: input.name.unwrap_or(character.name),
        description: input.description.unwrap_or(character.description),
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
