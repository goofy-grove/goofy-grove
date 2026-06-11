use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, sea_query,
};
use thiserror::Error;

use crate::platform::database::entities::{personas, prelude::Personas};

#[derive(Debug, Clone)]
pub struct Persona {
    pub uid: String,
    pub creator_id: String,
    pub name: String,
    pub description: String,
    pub avatar_id: Option<String>,
}

impl From<personas::Model> for Persona {
    fn from(model: personas::Model) -> Self {
        Self {
            uid: model.uid,
            creator_id: model.creator_id,
            name: model.name,
            description: model.description,
            avatar_id: model.avatar_uid,
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
    user_id: &str,
) -> Result<Vec<Persona>, LoadPersonaError> {
    let personas = Personas::find()
        .filter(personas::Column::CreatorId.eq(user_id))
        .all(connection)
        .await
        .map_err(|err| LoadPersonaError::InternalError(err.to_string()))?;

    Ok(personas.into_iter().map(Into::into).collect())
}

pub async fn load_persona(
    connection: &impl ConnectionTrait,
    persona_id: &str,
    user_id: &str,
) -> Result<Persona, LoadPersonaError> {
    Personas::find()
        .filter(personas::Column::Uid.eq(persona_id))
        .filter(personas::Column::CreatorId.eq(user_id))
        .one(connection)
        .await
        .map_err(|err| LoadPersonaError::InternalError(err.to_string()))?
        .map(Into::into)
        .ok_or(LoadPersonaError::NotFound)
}

pub async fn save_persona(
    connection: &impl ConnectionTrait,
    persona: Persona,
) -> Result<Persona, SavePersonaError> {
    let Persona {
        uid,
        creator_id,
        name,
        description,
        avatar_id: avatar_uid,
    } = persona;

    let new_persona = personas::ActiveModel {
        uid: Set(uid),
        creator_id: Set(creator_id),
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
                    personas::Column::CreatorId,
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
    persona_id: &str,
    user_id: &str,
) -> Result<(), DeletePersonaError> {
    let result = Personas::delete_many()
        .filter(personas::Column::Uid.eq(persona_id))
        .filter(personas::Column::CreatorId.eq(user_id))
        .exec(connection)
        .await
        .map_err(|err| DeletePersonaError::InternalError(err.to_string()))?;

    if result.rows_affected == 0 {
        return Err(DeletePersonaError::NotFound);
    }

    Ok(())
}
