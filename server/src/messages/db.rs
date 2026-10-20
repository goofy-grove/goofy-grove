use std::collections::HashMap;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use migration::OnConflict;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, SqlErr, Statement,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    character::Character,
    persona::Persona,
    platform::database::{
        PageData,
        entities::{characters, messages, personas},
    },
};

#[derive(Debug, Clone, Serialize)]
pub struct MessageInfo {
    pub uid: String,
    pub author_uid: Option<MessageAuthorUid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub chat_uid: String,
    pub reply_to_message_uid: Option<String>,
    pub is_removed: bool,
}

impl From<messages::Model> for MessageInfo {
    fn from(message: messages::Model) -> Self {
        let mut author_uid: Option<MessageAuthorUid> = None;

        if let Some(uid) = message.author_persona_uid {
            author_uid = Some(MessageAuthorUid::Persona(uid))
        } else if let Some(uid) = message.author_character_uid {
            author_uid = Some(MessageAuthorUid::Character(uid))
        }

        Self {
            uid: message.uid,
            author_uid,
            content: message.content,
            created_at: message.created_at,
            chat_uid: message.chat_uid,
            reply_to_message_uid: message.reply_to_message_uid,
            is_removed: message.is_removed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "uid", rename_all = "snake_case")]
pub enum MessageAuthorUid {
    Persona(String),
    Character(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub uid: String,
    pub author: Option<MessageAuthor>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub chat_uid: String,
    pub reply_to_message: Option<MessageInfo>,
    pub is_removed: bool,
}

impl
    From<(
        messages::Model,
        Option<personas::Model>,
        Option<characters::Model>,
        Option<MessageInfo>,
    )> for Message
{
    fn from(
        (message, persona, character, reply_to_message): (
            messages::Model,
            Option<personas::Model>,
            Option<characters::Model>,
            Option<MessageInfo>,
        ),
    ) -> Self {
        let mut author: Option<MessageAuthor> = None;

        if let Some(persona) = persona {
            author = Some(MessageAuthor::Persona(persona.into()))
        } else if let Some(character) = character {
            author = Some(MessageAuthor::Character(character.into()))
        }

        Self {
            uid: message.uid,
            author,
            content: message.content,
            created_at: message.created_at,
            chat_uid: message.chat_uid,
            reply_to_message,
            is_removed: message.is_removed,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MessageAuthor {
    Persona(Persona),
    Character(Character),
}

#[derive(Debug, Clone, Error)]
enum LoadMessageInfosError {
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Error)]
pub enum LoadMessageError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found")]
    NotFound,
}

#[derive(Debug, Clone, Error)]
pub enum LoadMessagesError {
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Error)]
pub enum LoadLatestMessagesByChat {
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Error)]
pub enum SaveMessageError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Chat or author not found")]
    NotFound,
}

async fn load_message_infos(
    connection: &impl ConnectionTrait,
    message_uids: Vec<String>,
) -> Result<HashMap<String, MessageInfo>, LoadMessageInfosError> {
    Ok(messages::Entity::find()
        .filter(
            messages::Column::Uid.is_in(message_uids.into_iter().unique().collect::<Vec<String>>()),
        )
        .all(connection)
        .await
        .map_err(|err| LoadMessageInfosError::Internal(err.to_string()))?
        .into_iter()
        .map(|model| (model.uid.clone(), model.into()))
        .collect())
}

pub async fn load_message(
    connection: &impl ConnectionTrait,
    message_uid: String,
) -> Result<Message, LoadMessageError> {
    let message = messages::Entity::find_by_id(message_uid)
        .find_also_related(personas::Entity)
        .find_also_related(characters::Entity)
        .one(connection)
        .await
        .map_err(|err| LoadMessageError::Internal(err.to_string()))?
        .ok_or(LoadMessageError::NotFound)?;

    let mut reply_to_message: Option<MessageInfo> = None;

    if let Some(uid) = message.0.reply_to_message_uid.as_ref() {
        reply_to_message = load_message_infos(connection, vec![uid.clone()])
            .await
            .map_err(|err| LoadMessageError::Internal(err.to_string()))?
            .remove(uid);
    }

    Ok((message.0, message.1, message.2, reply_to_message).into())
}

pub async fn load_latest_messages_by_chat(
    connection: &impl ConnectionTrait,
    chats_uids: Vec<String>,
) -> Result<HashMap<String, Message>, LoadLatestMessagesByChat> {
    // NOTE: This query retrieves the most recent messages for each passed chat.
    //
    // SQLite does not support DISTINCT ON, unlike PostgreSQL, so window functions are used here to ensure compatibility with both databases.
    //
    // Since SeaORM does not support window queries, two queries are made to maintain code readability:
    // - the first to retrieve all message identifiers,
    // - and the second to retrieve the rest of the information about the messages and the entities associated with them (characters and personas).
    let placeholders = std::iter::repeat("?")
        .take(chats_uids.len())
        .collect::<Vec<_>>()
        .join(", ");
    let messages_query = format!(
        r#"
            SELECT uid FROM (
                SELECT 
                    uid,
                    ROW_NUMBER() OVER (
                        PARTITION BY chat_uid 
                        ORDER BY created_at DESC, uid DESC
                    ) AS rn
                FROM messages
                WHERE chat_uid IN ({placeholders})
            ) AS ranked
            WHERE rn = 1
        "#
    );
    let values = chats_uids.into_iter().map(Into::into);
    let statement =
        Statement::from_sql_and_values(connection.get_database_backend(), &messages_query, values);

    let messages_uids: Vec<String> = connection
        .query_all_raw(statement)
        .await
        .map_err(|err| {
            LoadLatestMessagesByChat::Internal(format!(
                "Failed to load latest messages by chat: {}",
                err.to_string()
            ))
        })?
        .into_iter()
        .filter_map(|row| row.try_get_by_index(0).ok())
        .collect();

    let messages = messages::Entity::find()
        .filter(messages::Column::Uid.is_in(messages_uids))
        .find_also_related(personas::Entity)
        .find_also_related(characters::Entity)
        .all(connection)
        .await
        .map_err(|err| {
            LoadLatestMessagesByChat::Internal(format!(
                "Failed to load latest messages by chat: {}",
                err.to_string()
            ))
        })?;

    let reply_to_messages_uids: Vec<String> = messages
        .iter()
        .filter_map(|(message, _, _)| message.reply_to_message_uid.clone())
        .collect();

    let loaded_reply_to_messages_infos = load_message_infos(connection, reply_to_messages_uids)
        .await
        .map_err(|err| LoadLatestMessagesByChat::Internal(err.to_string()))?;

    let messages = messages
        .into_iter()
        .map(|(message, persona, character)| {
            let reply_to_message = message
                .reply_to_message_uid
                .as_ref()
                .and_then(|uid| loaded_reply_to_messages_infos.get(uid).cloned());

            Message::from((message, persona, character, reply_to_message))
        })
        .into_group_map_by(|message| message.chat_uid.clone())
        .into_iter()
        .filter_map(|(key, mut value)| value.pop().map(|inner| (key, inner)))
        .collect();

    Ok(messages)
}

pub async fn load_messages(
    connection: &impl ConnectionTrait,
    chat_uid: String,
    page_data: PageData,
) -> Result<Vec<Message>, LoadMessagesError> {
    let mut messages_cursor = messages::Entity::find()
        .filter(messages::Column::ChatUid.eq(chat_uid))
        .find_also_related(personas::Entity)
        .find_also_related(characters::Entity)
        .cursor_by((messages::Column::CreatedAt, messages::Column::Uid));

    messages_cursor.desc();

    if let Some(page) = page_data.next_page {
        messages_cursor.after(page);
    }

    let messages = messages_cursor
        .first(page_data.limit)
        .all(connection)
        .await
        .map_err(|err| LoadMessagesError::Internal(err.to_string()))?;

    let reply_to_messages_uids: Vec<String> = messages
        .iter()
        .filter_map(|(message, _, _)| message.reply_to_message_uid.clone())
        .collect();

    let loaded_reply_to_messages_infos = load_message_infos(connection, reply_to_messages_uids)
        .await
        .map_err(|err| LoadMessagesError::Internal(err.to_string()))?;

    let messages = messages
        .into_iter()
        .map(|(message, persona, character)| {
            let reply_to_message = message
                .reply_to_message_uid
                .as_ref()
                .and_then(|uid| loaded_reply_to_messages_infos.get(uid).cloned());

            (message, persona, character, reply_to_message).into()
        })
        .collect();

    Ok(messages)
}

pub async fn save_message(
    connection: &impl ConnectionTrait,
    message: MessageInfo,
) -> Result<MessageInfo, SaveMessageError> {
    let author_persona_uid =
        if let Some(MessageAuthorUid::Persona(uid)) = message.author_uid.clone() {
            Some(uid)
        } else {
            None
        };
    let author_character_uid = if let Some(MessageAuthorUid::Character(uid)) = message.author_uid {
        Some(uid)
    } else {
        None
    };

    let model = messages::ActiveModel {
        uid: Set(message.uid),
        content: Set(message.content),
        created_at: Set(message.created_at),
        author_persona_uid: Set(author_persona_uid),
        author_character_uid: Set(author_character_uid),
        chat_uid: Set(message.chat_uid),
        reply_to_message_uid: Set(message.reply_to_message_uid),
        is_removed: Set(message.is_removed),
    };

    let message = messages::Entity::insert(model)
        .on_conflict(
            OnConflict::column(messages::Column::Uid)
                .update_columns([messages::Column::Content, messages::Column::IsRemoved])
                .to_owned(),
        )
        .exec_with_returning(connection)
        .await
        .map_err(|err| match err.sql_err() {
            Some(SqlErr::ForeignKeyConstraintViolation(_)) => SaveMessageError::NotFound,
            _ => SaveMessageError::Internal(err.to_string()),
        })?;

    Ok(message.into())
}
