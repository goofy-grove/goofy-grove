use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, sea_query,
};
use thiserror::Error;

use crate::platform::database::entities::{characters, prelude::Characters};

#[derive(Debug, Clone)]
pub struct Character {
    pub uid: String,
    pub creator_id: String,
    pub name: String,
    pub description: String,
    pub avatar_uid: Option<String>,
}

impl From<characters::Model> for Character {
    fn from(model: characters::Model) -> Self {
        Self {
            uid: model.uid,
            creator_id: model.user_id,
            name: model.name,
            description: model.description,
            avatar_uid: model.avatar_uid,
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum LoadCharacterError {
    #[error("Character not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum SaveCharacterError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum DeleteCharacterError {
    #[error("Character not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn load_characters(
    connection: &impl ConnectionTrait,
    user_id: &str,
) -> Result<Vec<Character>, LoadCharacterError> {
    let models = Characters::find()
        .filter(characters::Column::UserId.eq(user_id))
        .all(connection)
        .await
        .map_err(|err| LoadCharacterError::InternalError(err.to_string()))?;

    Ok(models.into_iter().map(Into::into).collect())
}

pub async fn load_character(
    connection: &impl ConnectionTrait,
    character_id: &str,
    user_id: &str,
) -> Result<Character, LoadCharacterError> {
    let model = Characters::find()
        .filter(characters::Column::Uid.eq(character_id))
        .filter(characters::Column::UserId.eq(user_id))
        .one(connection)
        .await
        .map_err(|err| LoadCharacterError::InternalError(err.to_string()))?
        .ok_or(LoadCharacterError::NotFound)?;

    Ok(model.into())
}

pub async fn save_character(
    connection: &impl ConnectionTrait,
    character: Character,
) -> Result<Character, SaveCharacterError> {
    let Character {
        uid,
        creator_id,
        name,
        description,
        avatar_uid: avatar_id
    } = character;

    let active = characters::ActiveModel {
        uid: Set(uid),
        user_id: Set(creator_id),
        name: Set(name),
        description: Set(description),
        avatar_uid: Set(avatar_id),
    };

    let model = Characters::insert(active)
        .on_conflict(
            sea_query::OnConflict::column(characters::Column::Uid)
                .update_columns([
                    characters::Column::Name,
                    characters::Column::Description,
                    characters::Column::UserId,
                    characters::Column::AvatarUid,
                ])
                .to_owned(),
        )
        .exec_with_returning(connection)
        .await
        .map_err(|err| SaveCharacterError::InternalError(err.to_string()))?;

    Ok(model.into())
}

pub async fn delete_character(
    connection: &impl ConnectionTrait,
    character_id: &str,
    user_id: &str,
) -> Result<(), DeleteCharacterError> {
    let result = Characters::delete_many()
        .filter(characters::Column::Uid.eq(character_id))
        .filter(characters::Column::UserId.eq(user_id))
        .exec(connection)
        .await
        .map_err(|err| DeleteCharacterError::InternalError(err.to_string()))?;

    if result.rows_affected == 0 {
        return Err(DeleteCharacterError::NotFound);
    }

    Ok(())
}
