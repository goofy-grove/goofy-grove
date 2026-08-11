#![allow(unused)]

use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, Chat},
        events::invite::InviteToChatEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum InviteUserError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("User already in chat")]
    UserAlreadyInChat,

    #[error("User or chat not found")]
    UserOrChatNotFound,

    #[error("Forbidden")]
    Forbidden,
}

#[derive(Debug, Clone)]
pub struct InviteUserInput {
    pub chat_uid: String,
    pub user_uid: String,
    pub initiator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn invite_user(deps: &AppDeps, input: InviteUserInput) -> Result<Chat, InviteUserError> {
    let mut chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::InternalError(err) => InviteUserError::InternalError(err),
            db::LoadChatError::NotFound => InviteUserError::UserOrChatNotFound,
        })?;

    // NOTE: replace by acl in the future
    if chat.creator_uid != input.initiator_uid {
        return Err(InviteUserError::Forbidden);
    }

    let chat_member = db::join_user_to_chat(&deps.db, &input.chat_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            db::JoinUserToChatError::ChatOrUserNotFound => InviteUserError::UserOrChatNotFound,
            db::JoinUserToChatError::UserAlreadyInChat => InviteUserError::UserAlreadyInChat,
            db::JoinUserToChatError::InternalError(err) => InviteUserError::InternalError(err),
        })?;

    chat.members.push(chat_member.clone());
    chat.members.sort_by_key(|chat| chat.joined_at);

    deps.event_bus
        .publish(InviteToChatEvent {
            chat: chat.clone(),
            user: chat_member.user,
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(chat)
}
