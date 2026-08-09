use thiserror::Error;

use crate::{
    app::AppDeps,
    character::{db::character, events::types::CharacterDeletedEvent},
    file::{OrphanAvatarError, orphan_avatar_if_present},
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
    pub character_uid: String,
    pub user_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn delete_character(
    deps: &AppDeps,
    input: DeleteCharacterInput,
) -> Result<(), DeleteCharacterError> {
    let character = character::load_character(&deps.db, &input.character_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::NotFound => DeleteCharacterError::NotFound,
            character::LoadCharacterError::InternalError(message) => {
                DeleteCharacterError::InternalError(message)
            }
        })?;

    if let Err(OrphanAvatarError::InternalError(message)) =
        orphan_avatar_if_present(deps, character.avatar_uid.clone()).await
    {
        return Err(DeleteCharacterError::InternalError(message));
    }

    character::delete_character(&deps.db, &input.character_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            character::DeleteCharacterError::NotFound => DeleteCharacterError::NotFound,
            character::DeleteCharacterError::InternalError(message) => {
                DeleteCharacterError::InternalError(message)
            }
        })?;

    deps.event_bus
        .publish(CharacterDeletedEvent {
            character_uid: input.character_uid,
            creator_uid: input.user_uid,
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(())
}
