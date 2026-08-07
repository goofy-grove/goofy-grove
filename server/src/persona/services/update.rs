use thiserror::Error;

use crate::{
    app::AppDeps,
    file::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch},
    persona::{
        db::persona::{self, Persona},
        events::types::PersonaUpdatedEvent,
    },
    platform::events::EventPublisher,
    platform::types::PatchField,
};

#[derive(Debug, Clone, Error)]
pub enum UpdatePersonaError {
    #[error("Not found")]
    NotFound,

    #[error("Access denied")]
    AccessDenied,

    #[error("File not found")]
    FileNotFound,

    #[error("Invalid file status")]
    InvalidFileStatus,

    #[error("Invalid file scope")]
    InvalidFileScope,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct UpdatePersonaInput {
    pub persona_uid: String,
    pub user_uid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_uid: PatchField<String>,
    pub exclude_participants: Vec<String>,
}

pub async fn update_persona(
    deps: &AppDeps,
    input: UpdatePersonaInput,
) -> Result<Persona, UpdatePersonaError> {
    let persona = persona::load_persona(&deps.db, &input.persona_uid, &input.user_uid)
        .await
        .map_err(|err| match err {
            persona::LoadPersonaError::NotFound => UpdatePersonaError::NotFound,
            persona::LoadPersonaError::InternalError(message) => {
                UpdatePersonaError::InternalError(message)
            }
        })?;

    if persona.creator_uid != input.user_uid {
        return Err(UpdatePersonaError::AccessDenied);
    }

    let expected_scope = FileScope::PersonaAvatar {
        user_uid: input.user_uid.clone(),
        persona_uid: persona.uid.clone(),
    };

    let next_avatar_uid = apply_avatar_uid_patch(
        deps,
        persona.avatar_uid.clone(),
        input.avatar_uid,
        &expected_scope,
    )
    .await
    .map_err(|err| match err {
        ApplyAvatarPatchError::FileNotFound => UpdatePersonaError::FileNotFound,
        ApplyAvatarPatchError::InvalidFileStatus => UpdatePersonaError::InvalidFileStatus,
        ApplyAvatarPatchError::InvalidFileScope => UpdatePersonaError::InvalidFileScope,
        ApplyAvatarPatchError::InternalError(message) => UpdatePersonaError::InternalError(message),
    })?;

    let updated = Persona {
        uid: persona.uid,
        creator_uid: persona.creator_uid,
        name: input.name.unwrap_or(persona.name),
        description: input.description.unwrap_or(persona.description),
        avatar_uid: next_avatar_uid,
    };

    let saved = persona::save_persona(&deps.db, updated)
        .await
        .map_err(|err| UpdatePersonaError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(PersonaUpdatedEvent {
            persona: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
