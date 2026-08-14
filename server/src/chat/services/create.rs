#![allow(unused)]

use sea_orm::TransactionTrait;
use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, Chat, SaveChatPayload},
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
    pub description: String,
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

    let mut saved = db::save_chat(
        &transaction,
        SaveChatPayload {
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

    saved.members.push(member);

    deps.event_bus
        .publish(ChatCreatedEvent {
            chat: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
