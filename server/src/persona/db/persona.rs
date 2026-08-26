use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, sea_query,
};
use serde::Serialize;
use thiserror::Error;

use crate::platform::database::entities::{
    chat_members, chats, messages, personas, prelude::Personas,
};

#[derive(Debug, Clone, Serialize)]
pub struct Persona {
    pub uid: String,
    pub creator_uid: String,
    pub name: String,
    pub description: String,
    pub avatar_uid: Option<String>,
}

impl From<personas::Model> for Persona {
    fn from(model: personas::Model) -> Self {
        Self {
            uid: model.uid,
            creator_uid: model.creator_uid,
            name: model.name,
            description: model.description,
            avatar_uid: model.avatar_uid,
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum LoadPersonaError {
    #[error("Persona not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum SavePersonaError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum DeletePersonaError {
    #[error("Persona not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn load_personas(
    connection: &impl ConnectionTrait,
    user_uid: &str,
) -> Result<Vec<Persona>, LoadPersonaError> {
    let personas = Personas::find()
        .filter(personas::Column::CreatorUid.eq(user_uid))
        .all(connection)
        .await
        .map_err(|err| LoadPersonaError::InternalError(err.to_string()))?;

    Ok(personas.into_iter().map(Into::into).collect())
}

pub async fn load_persona(
    connection: &impl ConnectionTrait,
    persona_uid: &str,
    user_uid: &str,
) -> Result<Persona, LoadPersonaError> {
    Personas::find()
        .filter(personas::Column::Uid.eq(persona_uid))
        .filter(personas::Column::CreatorUid.eq(user_uid))
        .one(connection)
        .await
        .map_err(|err| LoadPersonaError::InternalError(err.to_string()))?
        .map(Into::into)
        .ok_or(LoadPersonaError::NotFound)
}

pub async fn is_visible_to_user(
    connection: &impl ConnectionTrait,
    persona_uid: &str,
    user_uid: &str,
) -> Result<bool, LoadPersonaError> {
    chat_members::Entity::find()
        .filter(chat_members::Column::UserUid.eq(user_uid))
        .join(JoinType::InnerJoin, chat_members::Relation::Chats.def())
        .join(JoinType::InnerJoin, chats::Relation::Messages.def())
        .filter(messages::Column::AuthorPersonaUid.eq(persona_uid))
        .one(connection)
        .await
        .map(|record| record.is_some())
        .map_err(|err| LoadPersonaError::InternalError(err.to_string()))
}

pub async fn save_persona(
    connection: &impl ConnectionTrait,
    persona: Persona,
) -> Result<Persona, SavePersonaError> {
    let Persona {
        uid,
        creator_uid,
        name,
        description,
        avatar_uid,
    } = persona;

    let new_persona = personas::ActiveModel {
        uid: Set(uid),
        creator_uid: Set(creator_uid),
        name: Set(name),
        description: Set(description),
        avatar_uid: Set(avatar_uid),
    };

    let saved_persona = Personas::insert(new_persona)
        .on_conflict(
            sea_query::OnConflict::column(personas::Column::Uid)
                .update_columns([
                    personas::Column::Name,
                    personas::Column::Description,
                    personas::Column::CreatorUid,
                    personas::Column::AvatarUid,
                ])
                .to_owned(),
        )
        .exec_with_returning(connection)
        .await
        .map_err(|err| SavePersonaError::InternalError(err.to_string()))?;

    Ok(saved_persona.into())
}

pub async fn delete_persona(
    connection: &impl ConnectionTrait,
    persona_uid: &str,
    user_uid: &str,
) -> Result<(), DeletePersonaError> {
    let result = Personas::delete_many()
        .filter(personas::Column::Uid.eq(persona_uid))
        .filter(personas::Column::CreatorUid.eq(user_uid))
        .exec(connection)
        .await
        .map_err(|err| DeletePersonaError::InternalError(err.to_string()))?;

    if result.rows_affected == 0 {
        return Err(DeletePersonaError::NotFound);
    }

    Ok(())
}
