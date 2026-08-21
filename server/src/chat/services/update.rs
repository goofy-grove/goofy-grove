use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, ChatInfo},
        events::updated::ChatUpdatedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum UpdateChatError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct UpdateChatInput {
    pub chat_uid: String,
    pub initiator_uid: String,
    pub name: String,
    pub exclude_participants: Vec<String>,
}

pub async fn update_chat(
    deps: &AppDeps,
    input: UpdateChatInput,
) -> Result<ChatInfo, UpdateChatError> {
    let mut chat = db::load_chat_info(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::InternalError(err) => UpdateChatError::Internal(err),
            db::LoadChatError::NotFound => UpdateChatError::NotFound,
        })?;

    if chat.creator_uid != input.initiator_uid {
        return Err(UpdateChatError::NotFound);
    }

    chat.name = input.name;

    let saved = db::save_chat(&deps.db, chat)
        .await
        .map_err(|err| UpdateChatError::Internal(err.to_string()))?;

    deps.event_bus
        .publish(ChatUpdatedEvent {
            chat: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
