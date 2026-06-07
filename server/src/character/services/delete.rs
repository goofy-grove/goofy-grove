use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{db::character, events::types::CharacterDeletedEvent},
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum DeleteCharacterError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct DeleteCharacterInput {
    pub id: String,
    pub user_id: String,
    pub exclude_participants: Vec<String>,
}

pub async fn delete_character(
    deps: &AppDeps,
    input: DeleteCharacterInput,
) -> Result<(), DeleteCharacterError> {
    character::delete_character(&deps.db, &input.id, &input.user_id)
        .await
        .map_err(|err| match err {
            character::DeleteCharacterError::NotFound => DeleteCharacterError::NotFound,
            character::DeleteCharacterError::InternalError(message) => {
                DeleteCharacterError::InternalError(message)
            }
        })?;

    deps.event_bus
        .publish(CharacterDeletedEvent {
            id: input.id,
            creator_id: input.user_id,
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(())
}
