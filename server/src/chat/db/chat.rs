#![allow(unused)]

use chrono::NaiveDateTime;
use itertools::Itertools;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, SqlErr, sea_query,
};
use serde::Serialize;
use thiserror::Error;

use crate::platform::database::entities::{
    characters, chat_characters, chat_members, chats, users,
};
use crate::{character::Character, user::User};

#[derive(Clone, Debug, Serialize)]
pub struct ChatMember {
    #[serde(flatten)]
    pub user: User,
    pub joined_at: NaiveDateTime,

    #[serde(skip)]
    pub chat_uid: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ChatCharacter {
    #[serde(flatten)]
    pub character: Character,
    pub connected_at: NaiveDateTime,

    #[serde(skip)]
    pub chat_uid: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Chat {
    pub uid: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub creator_uid: String,
    pub avatar_uid: Option<String>,
    pub members: Vec<ChatMember>,
    pub characters: Vec<ChatCharacter>,
}

impl From<chats::Model> for Chat {
    fn from(model: chats::Model) -> Self {
        Chat {
            uid: model.uid,
            name: model.name,
            created_at: model.created_at.naive_utc(),
            creator_uid: model.creator_uid,
            avatar_uid: model.avatar_uid,
            members: vec![],
            characters: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SaveChatPayload {
    pub uid: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub creator_uid: String,
    pub avatar_uid: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum SaveChatError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum LoadChatError {
    #[error("Chat not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum ConnectCharacterToChatError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Character or chat not found")]
    CharacterOrChatNotFound,

    #[error("Character already in chat")]
    CharacterAlreadyInChat,
}

#[derive(Debug, Clone, Error)]
pub enum DisconnectCharacterFromChatError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Character or chat not found")]
    CharacterOrChatNotFound,
}

#[derive(Debug, Clone, Error)]
pub enum JoinUserToChatError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Chat or user not found")]
    ChatOrUserNotFound,

    #[error("User already in chat")]
    UserAlreadyInChat,
}

#[derive(Debug, Clone, Error)]
pub enum LeaveUserFromChatError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Chat or user not found")]
    ChatOrUserNotFound,
}

pub async fn save_chat(
    connection: &impl ConnectionTrait,
    chat: SaveChatPayload,
) -> Result<Chat, SaveChatError> {
    let SaveChatPayload {
        uid,
        name,
        created_at,
        creator_uid,
        avatar_uid,
    } = chat;

    let chat = chats::ActiveModel {
        uid: Set(uid.clone()),
        name: Set(name),
        created_at: Set(created_at.and_utc()),
        creator_uid: Set(creator_uid),
        avatar_uid: Set(avatar_uid),
    };

    let model = chats::Entity::insert(chat)
        .on_conflict(
            sea_query::OnConflict::column(chats::Column::Uid)
                .update_columns([
                    chats::Column::Name,
                    chats::Column::CreatorUid,
                    chats::Column::AvatarUid,
                ])
                .to_owned(),
        )
        .exec_with_returning(connection)
        .await
        .map_err(|err| SaveChatError::InternalError(err.to_string()))?;

    Ok(model.into())
}

fn map_to_chat(
    chat_response: (chats::Model, Vec<(chat_members::Model, Vec<users::Model>)>),
) -> Chat {
    let (chat, members) = chat_response;

    let members = members
        .into_iter()
        .filter_map(|(member, user)| {
            let (user, _) = user.into_iter().next()?.into();

            Some(ChatMember {
                user,
                joined_at: member.joined_at.naive_utc(),
                chat_uid: chat.uid.clone(),
            })
        })
        .collect();

    Chat {
        uid: chat.uid,
        name: chat.name,
        creator_uid: chat.creator_uid,
        avatar_uid: chat.avatar_uid,
        created_at: chat.created_at.naive_utc(),
        members,
        characters: vec![],
    }
}

pub async fn load_user_chats(
    connection: &impl ConnectionTrait,
    user_uid: &str,
) -> Result<Vec<Chat>, LoadChatError> {
    let mut chats = chats::Entity::find()
        .has_related(
            chat_members::Entity,
            chat_members::Column::UserUid.eq(user_uid),
        )
        .find_also_related(chat_members::Entity)
        .and_also_related(users::Entity)
        .order_by_asc(chats::Column::CreatedAt)
        .order_by_asc(chat_members::Column::JoinedAt)
        .consolidate()
        .all(connection)
        .await
        .map_err(|err| LoadChatError::InternalError(err.to_string()))?
        .into_iter()
        .map(map_to_chat)
        .collect_vec();

    if chats.is_empty() {
        return Ok(chats);
    }

    let chat_ids = chats.iter().map(|chat| chat.uid.clone()).collect_vec();

    let mut chat_characters = chat_characters::Entity::find()
        .filter(chat_characters::Column::ChatUid.is_in(chat_ids))
        .find_also_related(characters::Entity)
        .order_by_asc(chat_characters::Column::ConnectedAt)
        .all(connection)
        .await
        .map_err(|err| LoadChatError::InternalError(err.to_string()))?
        .into_iter()
        .filter_map(|(chat_character, character)| {
            Some((
                chat_character.chat_uid.clone(),
                ChatCharacter {
                    character: character?.into(),
                    connected_at: chat_character.connected_at.naive_utc(),
                    chat_uid: chat_character.chat_uid,
                },
            ))
        })
        .into_group_map();

    for chat in chats.iter_mut() {
        chat.characters = chat_characters.remove(&chat.uid).unwrap_or_default();
    }

    Ok(chats)
}

pub async fn connect_character_to_chat(
    connection: &impl ConnectionTrait,
    character: ChatCharacter,
) -> Result<(), ConnectCharacterToChatError> {
    let result = chat_characters::Entity::insert(chat_characters::ActiveModel {
        chat_uid: Set(character.chat_uid),
        connected_at: Set(character.connected_at.and_utc()),
        character_uid: Set(character.character.uid),
    })
    .exec(connection)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(err) => match err.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(_)) => {
                Err(ConnectCharacterToChatError::CharacterAlreadyInChat)
            }
            Some(SqlErr::ForeignKeyConstraintViolation(_)) => {
                Err(ConnectCharacterToChatError::CharacterOrChatNotFound)
            }
            _ => Err(ConnectCharacterToChatError::InternalError(err.to_string())),
        },
    }
}

pub async fn disconnect_character_from_chat(
    connection: &impl ConnectionTrait,
    chat_uid: &str,
    character_uid: &str,
) -> Result<(), DisconnectCharacterFromChatError> {
    let result =
        chat_characters::Entity::delete_by_id((chat_uid.to_string(), character_uid.to_string()))
            .exec(connection)
            .await
            .map_err(|err| DisconnectCharacterFromChatError::InternalError(err.to_string()))?;

    if result.rows_affected != 1 {
        return Err(DisconnectCharacterFromChatError::CharacterOrChatNotFound);
    }

    Ok(())
}

pub async fn join_user_to_chat(
    connection: &impl ConnectionTrait,
    chat_uid: &str,
    user_uid: &str,
) -> Result<(), JoinUserToChatError> {
    let result = chat_members::Entity::insert(chat_members::ActiveModel {
        chat_uid: Set(chat_uid.to_string()),
        joined_at: NotSet,
        user_uid: Set(user_uid.to_string()),
    })
    .exec(connection)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(err) => match err.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(_)) => {
                Err(JoinUserToChatError::UserAlreadyInChat)
            }
            Some(SqlErr::ForeignKeyConstraintViolation(_)) => {
                Err(JoinUserToChatError::ChatOrUserNotFound)
            }
            _ => Err(JoinUserToChatError::InternalError(err.to_string())),
        },
    }
}

pub async fn leave_user_from_chat(
    connection: &impl ConnectionTrait,
    chat_uid: &str,
    user_uid: &str,
) -> Result<(), LeaveUserFromChatError> {
    let result = chat_members::Entity::delete_by_id((chat_uid.to_string(), user_uid.to_string()))
        .exec(connection)
        .await
        .map_err(|err| LeaveUserFromChatError::InternalError(err.to_string()))?;

    if result.rows_affected != 1 {
        return Err(LeaveUserFromChatError::ChatOrUserNotFound);
    }

    Ok(())
}
