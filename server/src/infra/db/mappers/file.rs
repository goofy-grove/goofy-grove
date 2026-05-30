use gg_core::domain::prelude::*;

use crate::infra::db::entities::files;

pub const SCOPE_USER_AVATAR: &str = "user_avatar";
pub const SCOPE_PERSONA_AVATAR: &str = "persona_avatar";

pub const STATUS_CREATED: &str = "created";
pub const STATUS_ACTIVATED: &str = "activated";
pub const STATUS_ORPHANED: &str = "orphaned";

pub fn scope_to_db(scope: &FileScope) -> (String, String, Option<String>) {
    match scope {
        FileScope::UserAvatar { user_id } => {
            (SCOPE_USER_AVATAR.to_string(), user_id.inner().clone(), None)
        }
        FileScope::PersonaAvatar {
            user_id,
            persona_id,
        } => (
            SCOPE_PERSONA_AVATAR.to_string(),
            user_id.inner().clone(),
            Some(persona_id.inner().clone()),
        ),
    }
}

pub fn scope_from_db(
    scope_kind: &str,
    scope_owner_id: &str,
    scope_entity_id: Option<&str>,
) -> Result<FileScope, String> {
    let user_id = UserId::try_new(scope_owner_id.to_string()).map_err(|err| err.to_string())?;

    match scope_kind {
        SCOPE_USER_AVATAR => Ok(FileScope::UserAvatar { user_id }),
        SCOPE_PERSONA_AVATAR => {
            let persona_id = scope_entity_id.ok_or_else(|| "internal error".to_string())?;
            Ok(FileScope::PersonaAvatar {
                user_id,
                persona_id: PersonaId::try_new(persona_id.to_string())
                    .map_err(|err| err.to_string())?,
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

pub fn file_meta_from_model(model: files::Model) -> Result<FileMeta, String> {
    Ok(FileMeta {
        id: FileId::try_new(model.uid).map_err(|err| err.to_string())?,
        filename: Filename::try_new(model.filename).map_err(|err| err.to_string())?,
        uploaded_by: UserId::try_new(model.uploaded_by).map_err(|err| err.to_string())?,
        scope: scope_from_db(
            &model.scope_kind,
            &model.scope_owner_id,
            model.scope_entity_id.as_deref(),
        )?,
        uploaded_at: UploadedAt::try_new(model.uploaded_at.and_utc().timestamp())
            .map_err(|err| err.to_string())?,
        status: status_from_db(&model.status)?,
        original_name: FileOriginalName::try_new(model.original_name)
            .map_err(|err| err.to_string())?,
        content_type: FileContentType::try_new(model.content_type)
            .map_err(|err| err.to_string())?,
        size: FileSize::try_new(model.size as usize).map_err(|err| err.to_string())?,
    })
}
