#![allow(unused)]

use crate::{
    app::AppDeps,
    chat::{
        db::{self, ChatInfo},
        events::updated::ChatUpdatedEvent,
    },
    file::{
        FileScope, ReplaceAvatarError, ReplaceAvatarInput, orphan_avatar_if_present, replace_avatar,
    },
    platform::events::EventPublisher,
};

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SetChatAvatarError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    ReplaceAvatar(#[from] ReplaceAvatarError),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Error)]
pub enum ClearChatAvatarError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone)]
pub struct SetChatAvatarInput {
    pub chat_uid: String,
    pub initiator_uid: String,
    pub content_type: String,
    pub original_name: String,
    pub content: Vec<u8>,
    pub exclude_participants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClearChatAvatarInput {
    pub chat_uid: String,
    pub initiator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn set_chat_avatar(
    deps: &AppDeps,
    input: SetChatAvatarInput,
) -> Result<ChatInfo, SetChatAvatarError> {
    let mut chat = db::load_chat_info(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => SetChatAvatarError::NotFound,
            db::LoadChatError::InternalError(message) => SetChatAvatarError::Internal(message),
        })?;

    if chat.creator_uid != input.initiator_uid {
        return Err(SetChatAvatarError::NotFound);
    }

    let new_avatar_uid = replace_avatar(
        deps,
        ReplaceAvatarInput {
            content_type: input.content_type,
            original_name: input.original_name,
            scope: FileScope::ChatAvatar {
                user_uid: input.initiator_uid.clone(),
                chat_uid: input.chat_uid,
            },
            content: input.content,
            current_avatar_uid: chat.avatar_uid.clone(),
        },
        &input.initiator_uid,
    )
    .await?;

    chat.avatar_uid = Some(new_avatar_uid);

    let saved = db::save_chat(&deps.db, chat)
        .await
        .map_err(|err| SetChatAvatarError::Internal(err.to_string()))?;

    deps.event_bus
        .publish(ChatUpdatedEvent {
            chat: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}

pub async fn clear_chat_avatar(
    deps: &AppDeps,
    input: ClearChatAvatarInput,
) -> Result<ChatInfo, ClearChatAvatarError> {
    let mut chat = db::load_chat_info(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => ClearChatAvatarError::NotFound,
            db::LoadChatError::InternalError(message) => ClearChatAvatarError::Internal(message),
        })?;

    if chat.creator_uid != input.initiator_uid {
        return Err(ClearChatAvatarError::NotFound);
    }

    orphan_avatar_if_present(deps, chat.avatar_uid.clone())
        .await
        .map_err(|err| ClearChatAvatarError::Internal(err.to_string()))?;

    chat.avatar_uid = None;

    let saved = db::save_chat(&deps.db, chat)
        .await
        .map_err(|err| ClearChatAvatarError::Internal(err.to_string()))?;

    deps.event_bus
        .publish(ChatUpdatedEvent {
            chat: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
