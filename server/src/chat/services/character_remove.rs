#![allow(unused)]

use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, ChatCharacter},
        events::character_removed::CharacterRemovedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum RemoveCharacterError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Character or chat not found")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct RemoveCharacterInput {
    pub chat_uid: String,
    pub character_uid: String,
    pub initiator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn remove_character(
    deps: &AppDeps,
    input: RemoveCharacterInput,
) -> Result<ChatCharacter, RemoveCharacterError> {
    let mut chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => RemoveCharacterError::NotFound,
            db::LoadChatError::InternalError(err) => RemoveCharacterError::Internal(err),
        })?;

    if chat.creator_uid != input.initiator_uid {
        return Err(RemoveCharacterError::NotFound);
    }

    let character = chat
        .characters
        .extract_if(.., |character| {
            character.character.uid == input.character_uid
        })
        .next()
        .ok_or(RemoveCharacterError::NotFound)?;

    db::remove_character_from_chat(&deps.db, &chat.uid, &character.character.uid)
        .await
        .map_err(|err| match err {
            db::RemoveCharacterFromChatError::CharacterOrChatNotFound => {
                RemoveCharacterError::NotFound
            }
            db::RemoveCharacterFromChatError::InternalError(err) => {
                RemoveCharacterError::Internal(err)
            }
        })?;

    deps.event_bus
        .publish(CharacterRemovedEvent {
            character: character.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(character)
}
