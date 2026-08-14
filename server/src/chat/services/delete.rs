#![allow(unused)]

use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{db, events::deleted::ChatDeletedEvent},
    file::{OrphanAvatarError, orphan_avatar_if_present},
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum DeleteChatError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone)]
pub struct DeleteChatInput {
    pub chat_uid: String,
    pub user_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn delete_chat(deps: &AppDeps, input: DeleteChatInput) -> Result<(), DeleteChatError> {
    let chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => DeleteChatError::NotFound,
            db::LoadChatError::InternalError(message) => DeleteChatError::Internal(message),
        })?;

    if chat.creator_uid != input.user_uid {
        return Err(DeleteChatError::NotFound);
    }

    if let Err(OrphanAvatarError::InternalError(message)) =
        orphan_avatar_if_present(deps, chat.avatar_uid.clone()).await
    {
        return Err(DeleteChatError::Internal(message));
    }

    db::delete_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::DeleteChatError::InternalError(message) => DeleteChatError::Internal(message),
            db::DeleteChatError::NotFound => DeleteChatError::NotFound,
        })?;

    deps.event_bus
        .publish(ChatDeletedEvent {
            chat_uid: input.chat_uid,
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(())
}
