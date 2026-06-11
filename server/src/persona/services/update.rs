use thiserror::Error;

use crate::{
    app::AppDeps,
    file::public::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch},
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
    pub id: String,
    pub user_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_uid: PatchField<String>,
    pub exclude_participants: Vec<String>,
}

pub async fn update_persona(
    deps: &AppDeps,
    input: UpdatePersonaInput,
) -> Result<Persona, UpdatePersonaError> {
    let persona = persona::load_persona(&deps.db, &input.id, &input.user_id)
        .await
        .map_err(|err| match err {
            persona::LoadPersonaError::NotFound => UpdatePersonaError::NotFound,
            persona::LoadPersonaError::InternalError(message) => {
                UpdatePersonaError::InternalError(message)
            }
        })?;

    if persona.creator_id != input.user_id {
        return Err(UpdatePersonaError::AccessDenied);
    }

    let expected_scope = FileScope::PersonaAvatar {
        user_id: input.user_id.clone(),
        persona_id: persona.uid.clone(),
    };

    let next_avatar_uid = apply_avatar_uid_patch(
        deps,
        persona.avatar_id.clone(),
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
        creator_id: persona.creator_id,
        name: input.name.unwrap_or(persona.name),
        description: input.description.unwrap_or(persona.description),
        avatar_id: next_avatar_uid,
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
