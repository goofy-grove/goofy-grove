use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{
        db::character::{self, Character},
        events::types::CharacterCreatedEvent,
    },
    platform::{events::EventPublisher, util},
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
    pub creator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn create_character(
    deps: &AppDeps,
    input: CreateCharacterInput,
) -> Result<Character, CreateCharacterError> {
    let uid = util::uid_generator::generate_uid("character");

    let character = Character {
        uid,
        creator_uid: input.creator_uid,
        name: input.name,
        description: input.description,
        avatar_uid: None,
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
