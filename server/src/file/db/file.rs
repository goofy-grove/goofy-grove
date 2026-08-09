use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, sea_query::Expr,
};
use thiserror::Error;

use crate::platform::database::entities::{files, prelude::Files};

pub const SCOPE_USER_AVATAR: &str = "user_avatar";
pub const SCOPE_PERSONA_AVATAR: &str = "persona_avatar";
pub const SCOPE_CHARACTER_AVATAR: &str = "character_avatar";

pub const STATUS_CREATED: &str = "created";
pub const STATUS_ACTIVATED: &str = "activated";
pub const STATUS_ORPHANED: &str = "orphaned";

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileScope {
    UserAvatar {
        user_uid: String,
    },
    PersonaAvatar {
        user_uid: String,
        persona_uid: String,
    },
    CharacterAvatar {
        user_uid: String,
        character_uid: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileStatus {
    Created,
    Activated,
    Orphaned,
}

#[derive(Debug, Clone)]
pub struct FileMeta {
    pub uid: String,
    pub filename: String,
    pub uploaded_by_uid: String,
    pub scope: FileScope,
    pub uploaded_at: DateTime<Utc>,
    pub status: FileStatus,
    pub original_name: String,
    pub content_type: String,
    pub size: usize,
}

#[derive(Debug, Clone, Error)]
pub enum LoadFileError {
    #[error("File not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum SaveFileError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum UpdateFileStatusError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub fn scope_to_db(scope: &FileScope) -> (String, String, Option<String>) {
    match scope {
        FileScope::UserAvatar { user_uid } => {
            (SCOPE_USER_AVATAR.to_string(), user_uid.clone(), None)
        }
        FileScope::PersonaAvatar {
            user_uid,
            persona_uid,
        } => (
            SCOPE_PERSONA_AVATAR.to_string(),
            user_uid.clone(),
            Some(persona_uid.clone()),
        ),
        FileScope::CharacterAvatar {
            user_uid,
            character_uid,
        } => (
            SCOPE_CHARACTER_AVATAR.to_string(),
            user_uid.clone(),
            Some(character_uid.clone()),
        ),
    }
}

pub fn scope_from_db(
    scope_kind: &str,
    scope_owner_uid: &str,
    scope_entity_uid: Option<&str>,
) -> Result<FileScope, String> {
    match scope_kind {
        SCOPE_USER_AVATAR => Ok(FileScope::UserAvatar {
            user_uid: scope_owner_uid.to_string(),
        }),
        SCOPE_PERSONA_AVATAR => {
            let persona_uid = scope_entity_uid.ok_or_else(|| "internal error".to_string())?;

            Ok(FileScope::PersonaAvatar {
                user_uid: scope_owner_uid.to_string(),
                persona_uid: persona_uid.to_string(),
            })
        }
        SCOPE_CHARACTER_AVATAR => {
            let character_uid = scope_entity_uid.ok_or_else(|| "internal error".to_string())?;

            Ok(FileScope::CharacterAvatar {
                user_uid: scope_owner_uid.to_string(),
                character_uid: character_uid.to_string(),
            })
        }
        _ => Err("internal error".to_string()),
    }
}

pub fn status_to_db(status: &FileStatus) -> &'static str {
    match status {
        FileStatus::Created => STATUS_CREATED,
        FileStatus::Activated => STATUS_ACTIVATED,
        FileStatus::Orphaned => STATUS_ORPHANED,
    }
}

pub fn status_from_db(value: &str) -> Result<FileStatus, String> {
    match value {
        STATUS_CREATED => Ok(FileStatus::Created),
        STATUS_ACTIVATED => Ok(FileStatus::Activated),
        STATUS_ORPHANED => Ok(FileStatus::Orphaned),
        _ => Err("internal error".to_string()),
    }
}

impl TryFrom<files::Model> for FileMeta {
    type Error = String;

    fn try_from(model: files::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: model.uid,
            filename: model.filename,
            uploaded_by_uid: model.uploaded_by_uid,
            scope: scope_from_db(
                &model.scope_kind,
                &model.scope_owner_uid,
                model.scope_entity_uid.as_deref(),
            )?,
            uploaded_at: model.uploaded_at.and_utc(),
            status: status_from_db(&model.status)?,
            original_name: model.original_name,
            content_type: model.content_type,
            size: model.size as usize,
        })
    }
}

pub async fn load_file(
    connection: &impl ConnectionTrait,
    uid: &str,
) -> Result<FileMeta, LoadFileError> {
    let model = Files::find_by_id(uid)
        .one(connection)
        .await
        .map_err(|err| LoadFileError::InternalError(err.to_string()))?
        .ok_or(LoadFileError::NotFound)?;

    model.try_into().map_err(LoadFileError::InternalError)
}

pub async fn save_file(
    connection: &impl ConnectionTrait,
    meta: &FileMeta,
) -> Result<(), SaveFileError> {
    let (scope_kind, scope_owner_uid, scope_entity_uid) = scope_to_db(&meta.scope);

    let active = files::ActiveModel {
        uid: Set(meta.uid.clone()),
        filename: Set(meta.filename.clone()),
        uploaded_by_uid: Set(meta.uploaded_by_uid.clone()),
        scope_kind: Set(scope_kind),
        scope_owner_uid: Set(scope_owner_uid),
        scope_entity_uid: Set(scope_entity_uid),
        uploaded_at: Set(meta.uploaded_at.naive_utc()),
        status: Set(status_to_db(&meta.status).to_string()),
        original_name: Set(meta.original_name.clone()),
        content_type: Set(meta.content_type.clone()),
        size: Set(meta.size as i64),
    };

    Files::insert(active)
        .exec(connection)
        .await
        .map_err(|err| SaveFileError::InternalError(err.to_string()))?;

    Ok(())
}

pub async fn activate_file(
    connection: &impl ConnectionTrait,
    uid: &str,
) -> Result<(), UpdateFileStatusError> {
    Files::update_many()
        .col_expr(
            files::Column::Status,
            Expr::value(STATUS_ACTIVATED.to_string()),
        )
        .filter(files::Column::Uid.eq(uid))
        .exec(connection)
        .await
        .map_err(|err| UpdateFileStatusError::InternalError(err.to_string()))?;

    Ok(())
}

pub async fn orphan_file(
    connection: &impl ConnectionTrait,
    uid: &str,
) -> Result<(), UpdateFileStatusError> {
    Files::update_many()
        .col_expr(
            files::Column::Status,
            Expr::value(STATUS_ORPHANED.to_string()),
        )
        .filter(files::Column::Uid.eq(uid))
        .exec(connection)
        .await
        .map_err(|err| UpdateFileStatusError::InternalError(err.to_string()))?;

    Ok(())
}
