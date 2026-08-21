use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{self, LoadCharacterError},
    chat::{
        db::{self, ChatCharacter},
        events::character_added::CharacterAddedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum AddCharacterError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Character or chat not found")]
    NotFound,

    #[error("Character already in chat")]
    AlreadyInChat,
}

#[derive(Debug, Clone)]
pub struct AddCharacterInput {
    pub chat_uid: String,
    pub character_uid: String,
    pub initiator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn add_character(
    deps: &AppDeps,
    input: AddCharacterInput,
) -> Result<ChatCharacter, AddCharacterError> {
    let chat = db::load_chat_info(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => AddCharacterError::NotFound,
            db::LoadChatError::InternalError(err) => AddCharacterError::Internal(err),
        })?;

    if chat.creator_uid != input.initiator_uid {
        return Err(AddCharacterError::NotFound);
    }

    let character = character::get_by_uid(&deps.db, &input.character_uid)
        .await
        .map_err(|err| match err {
            LoadCharacterError::NotFound => AddCharacterError::NotFound,
            LoadCharacterError::InternalError(err) => AddCharacterError::Internal(err),
        })?;

    let character = db::add_character_to_chat(&deps.db, character, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::AddCharacterToChatError::CharacterAlreadyInChat => AddCharacterError::AlreadyInChat,
            db::AddCharacterToChatError::CharacterOrChatNotFound => AddCharacterError::NotFound,
            db::AddCharacterToChatError::InternalError(err) => AddCharacterError::Internal(err),
        })?;

    deps.event_bus
        .publish(CharacterAddedEvent {
            character: character.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(character)
}
