use thiserror::Error;

use crate::{
    app::AppDeps,
    file::{
        FileScope, OrphanAvatarError, ReplaceAvatarError, ReplaceAvatarInput,
        orphan_avatar_if_present, replace_avatar,
    },
    persona::{
        db::persona::{self, Persona},
        events::types::PersonaUpdatedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum SetPersonaAvatarError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    ReplaceAvatar(#[from] ReplaceAvatarError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Error)]
pub enum ClearPersonaAvatarError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct SetPersonaAvatarInput {
    pub persona_uid: String,
    pub user_uid: String,
    pub content_type: String,
    pub original_name: String,
    pub content: Vec<u8>,
    pub exclude_participants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClearPersonaAvatarInput {
    pub persona_uid: String,
    pub user_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn set_persona_avatar(
    deps: &AppDeps,
    input: SetPersonaAvatarInput,
) -> Result<Persona, SetPersonaAvatarError> {
    let persona = persona::load_persona(&deps.db, &input.persona_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            persona::LoadPersonaError::NotFound => SetPersonaAvatarError::NotFound,
            persona::LoadPersonaError::InternalError(message) => {
                SetPersonaAvatarError::InternalError(message)
            }
        })?;

    let avatar_uid = replace_avatar(
        deps,
        ReplaceAvatarInput {
            content_type: input.content_type,
            original_name: input.original_name,
            scope: FileScope::PersonaAvatar {
                user_uid: input.user_uid.clone(),
                persona_uid: persona.uid.clone(),
            },
            content: input.content,
            current_avatar_uid: persona.avatar_uid.clone(),
        },
        &input.user_uid,
    )
    .await?;

    let updated = Persona {
        uid: persona.uid,
        creator_uid: persona.creator_uid,
        name: persona.name,
        description: persona.description,
        avatar_uid: Some(avatar_uid),
    };

    let saved = persona::save_persona(&deps.db, updated)
        .await
        .map_err(|err| SetPersonaAvatarError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(PersonaUpdatedEvent {
            persona: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}

pub async fn clear_persona_avatar(
    deps: &AppDeps,
    input: ClearPersonaAvatarInput,
) -> Result<Persona, ClearPersonaAvatarError> {
    let persona = persona::load_persona(&deps.db, &input.persona_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            persona::LoadPersonaError::NotFound => ClearPersonaAvatarError::NotFound,
            persona::LoadPersonaError::InternalError(message) => {
                ClearPersonaAvatarError::InternalError(message)
            }
        })?;

    if let Err(OrphanAvatarError::InternalError(message)) =
        orphan_avatar_if_present(deps, persona.avatar_uid.clone()).await
    {
        return Err(ClearPersonaAvatarError::InternalError(message));
    }

    let updated = Persona {
        uid: persona.uid,
        creator_uid: persona.creator_uid,
        name: persona.name,
        description: persona.description,
        avatar_uid: None,
    };

    let saved = persona::save_persona(&deps.db, updated)
        .await
        .map_err(|err| ClearPersonaAvatarError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(PersonaUpdatedEvent {
            persona: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
