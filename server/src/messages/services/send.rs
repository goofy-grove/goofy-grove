use thiserror::Error;

use crate::{
    app::AppDeps,
    messages::{
        db::{self, Message, MessageAuthorUid, MessageInfo, SaveMessageError},
        events::message_sended::MessageSendedEvent,
    },
    platform::{events::EventPublisher, util::uid_generator::generate_uid},
};

#[derive(Debug, Clone)]
pub struct SendInput {
    pub chat_uid: String,
    pub content: String,
    pub author_uid: MessageAuthorUid,
    pub reply_to_message_uid: Option<String>,
    pub exclude_participants: Vec<String>,
}

#[derive(Debug, Clone, Error)]
pub enum SendMessageError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Author or chat or reply message not found")]
    NotFound,
}

pub async fn send(deps: &AppDeps, input: SendInput) -> Result<Message, SendMessageError> {
    let message = db::save_message(
        &deps.db,
        MessageInfo {
            uid: generate_uid("message"),
            content: input.content,
            author_uid: Some(input.author_uid),
            chat_uid: input.chat_uid,
            created_at: chrono::Utc::now().naive_utc(),
            is_removed: false,
            reply_to_message_uid: input.reply_to_message_uid,
        },
    )
    .await
    .map_err(|err| match err {
        SaveMessageError::NotFound => SendMessageError::NotFound,
        SaveMessageError::Internal(err) => SendMessageError::Internal(err),
    })?;

    let message = db::load_message(&deps.db, message.uid)
        .await
        .map_err(|err| SendMessageError::Internal(err.to_string()))?;

    deps.event_bus
        .publish(MessageSendedEvent {
            message: message.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(message)
}
