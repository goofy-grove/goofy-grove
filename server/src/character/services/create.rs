use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{
        db::character::{self, Character},
        events::types::CharacterCreatedEvent,
    },
    platform::events::EventPublisher,
    platform::util,
};

#[derive(Debug, Clone, Error)]
pub enum CreateCharacterError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct CreateCharacterInput {
    pub name: String,
    pub description: String,
    pub creator_id: String,
    pub exclude_participants: Vec<String>,
}

pub async fn create_character(
    deps: &AppDeps,
    input: CreateCharacterInput,
) -> Result<Character, CreateCharacterError> {
    let character = Character {
        uid: util::id_generator::generate_id("character"),
        creator_id: input.creator_id,
        name: input.name,
        description: input.description,
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
