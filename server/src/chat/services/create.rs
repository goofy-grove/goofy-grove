use sea_orm::TransactionTrait;
use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, Chat, ChatInfo},
        events::created::ChatCreatedEvent,
    },
    platform::{events::EventPublisher, util},
};

#[derive(Debug, Clone, Error)]
pub enum CreateChatError {
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone)]
pub struct CreateChatInput {
    pub name: String,
    pub creator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn create_chat(deps: &AppDeps, input: CreateChatInput) -> Result<Chat, CreateChatError> {
    let uid = util::uid_generator::generate_uid("chat");
    let transaction = deps
        .db
        .begin()
        .await
        .map_err(|err| CreateChatError::Internal(err.to_string()))?;

    let saved = db::save_chat(
        &transaction,
        ChatInfo {
            uid,
            name: input.name,
            created_at: chrono::Utc::now().naive_utc(),
            creator_uid: input.creator_uid,
            avatar_uid: None,
        },
    )
    .await
    .map_err(|err| CreateChatError::Internal(err.to_string()))?;

    let member = db::add_user_to_chat(&transaction, &saved.uid, &saved.creator_uid)
        .await
        .map_err(|err| CreateChatError::Internal(err.to_string()))?;

    transaction
        .commit()
        .await
        .map_err(|err| CreateChatError::Internal(err.to_string()))?;

    let chat = Chat {
        uid: saved.uid,
        name: saved.name,
        creator_uid: saved.creator_uid,
        created_at: saved.created_at,
        avatar_uid: saved.avatar_uid,
        members: vec![member],
        characters: vec![],
    };

    deps.event_bus
        .publish(ChatCreatedEvent {
            chat: chat.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(chat)
}
