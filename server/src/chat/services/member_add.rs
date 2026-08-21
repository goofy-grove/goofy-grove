use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::{
        db::{self, ChatMember},
        events::member_added::MemberAddedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum AddMemberError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("User already in chat")]
    AlreadyInChat,

    #[error("User or chat not found")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct AddMemberInput {
    pub chat_uid: String,
    pub user_uid: String,
    pub initiator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn add_member(
    deps: &AppDeps,
    input: AddMemberInput,
) -> Result<ChatMember, AddMemberError> {
    let mut chat = db::load_chat(&deps.db, &input.chat_uid)
        .await
        .map_err(|err| match err {
            db::LoadChatError::InternalError(err) => AddMemberError::Internal(err),
            db::LoadChatError::NotFound => AddMemberError::NotFound,
        })?;

    // NOTE: replace by acl in the future
    if chat.creator_uid != input.initiator_uid {
        return Err(AddMemberError::NotFound);
    }

    let chat_member = db::add_user_to_chat(&deps.db, &input.chat_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            db::AddUserToChatError::ChatOrUserNotFound => AddMemberError::NotFound,
            db::AddUserToChatError::UserAlreadyInChat => AddMemberError::AlreadyInChat,
            db::AddUserToChatError::InternalError(err) => AddMemberError::Internal(err),
        })?;

    chat.members.push(chat_member.clone());
    chat.members.sort_by_key(|chat| chat.joined_at);

    deps.event_bus
        .publish(MemberAddedEvent {
            chat: chat.clone(),
            member: chat_member.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(chat_member)
}
