#![allow(unused)]

use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, Chat, ChatMember},
        events::member_removed::MemberRemovedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum RemoveMemberFromChatError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("User or chat not found")]
    UserOrChatNotFound,
}

#[derive(Debug, Clone)]
pub struct RemoveUserFromChatInput {
    pub chat_uid: String,
    pub user_uid: String,
    pub initiator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn remove_member_from_chat(
    deps: &AppDeps,
    input: RemoveUserFromChatInput,
) -> Result<ChatMember, RemoveMemberFromChatError> {
    let mut chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::InternalError(err) => RemoveMemberFromChatError::InternalError(err),
            db::LoadChatError::NotFound => RemoveMemberFromChatError::UserOrChatNotFound,
        })?;

    // NOTE: replace by acl in the future
    if chat.creator_uid != input.initiator_uid || chat.creator_uid == input.user_uid {
        return Err(RemoveMemberFromChatError::Forbidden);
    }

    let member = chat
        .members
        .extract_if(.., |member| member.user.uid == input.user_uid)
        .next()
        .ok_or(RemoveMemberFromChatError::UserOrChatNotFound)?;

    db::remove_user_from_chat(&deps.db, &input.chat_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            db::RemoveUserFromChatError::InternalError(err) => {
                RemoveMemberFromChatError::InternalError(err)
            }
            db::RemoveUserFromChatError::ChatOrUserNotFound => {
                RemoveMemberFromChatError::UserOrChatNotFound
            }
        })?;

    deps.event_bus
        .publish(MemberRemovedEvent {
            member: member.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(member)
}
