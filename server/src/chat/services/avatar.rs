#![allow(unused)]

use crate::{
    app::AppDeps,
    chat::{db, events::updated::ChatUpdatedEvent},
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
    pub user_uid: String,
    pub content_type: String,
    pub original_name: String,
    pub content: Vec<u8>,
    pub exclude_participants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClearChatAvatarInput {
    pub chat_uid: String,
    pub user_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn set_chat_avatar(
    deps: &AppDeps,
    input: SetChatAvatarInput,
) -> Result<db::Chat, SetChatAvatarError> {
    let mut chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => SetChatAvatarError::NotFound,
            db::LoadChatError::InternalError(message) => SetChatAvatarError::Internal(message),
        })?;

    let new_avatar_uid = replace_avatar(
        deps,
        ReplaceAvatarInput {
            content_type: input.content_type,
            original_name: input.original_name,
            scope: FileScope::ChatAvatar {
                user_uid: input.user_uid.clone(),
                chat_uid: input.chat_uid,
            },
            content: input.content,
            current_avatar_uid: chat.avatar_uid.clone(),
        },
        &input.user_uid,
    )
    .await?;

    chat.avatar_uid = Some(new_avatar_uid);

    let saved = db::save_chat(
        &deps.db,
        db::SaveChatPayload {
            uid: chat.uid,
            name: chat.name,
            created_at: chat.created_at,
            creator_uid: chat.creator_uid,
            avatar_uid: chat.avatar_uid,
        },
    )
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
) -> Result<db::Chat, ClearChatAvatarError> {
    let mut chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::NotFound => ClearChatAvatarError::NotFound,
            db::LoadChatError::InternalError(message) => ClearChatAvatarError::Internal(message),
        })?;

    orphan_avatar_if_present(deps, chat.avatar_uid.clone())
        .await
        .map_err(|err| ClearChatAvatarError::Internal(err.to_string()))?;

    chat.avatar_uid = None;

    let saved = db::save_chat(
        &deps.db,
        db::SaveChatPayload {
            uid: chat.uid,
            name: chat.name,
            created_at: chat.created_at,
            creator_uid: chat.creator_uid,
            avatar_uid: chat.avatar_uid,
        },
    )
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
