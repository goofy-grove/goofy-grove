use thiserror::Error;

use crate::{
    app::AppDeps,
    file::public::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch},
    persona::{
        db::persona::{self, Persona},
        events::types::PersonaCreatedEvent,
    },
    platform::events::EventPublisher,
    platform::{types::PatchField, util},
};

#[derive(Debug, Clone, Error)]
pub enum CreatePersonaError {
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
pub struct CreatePersonaInput {
    pub name: String,
    pub description: String,
    pub creator_id: String,
    pub avatar_uid: Option<String>,
    pub exclude_participants: Vec<String>,
}

pub async fn create_persona(
    deps: &AppDeps,
    input: CreatePersonaInput,
) -> Result<Persona, CreatePersonaError> {
    let uid = util::id_generator::generate_id("persona");

    let avatar_uid = if let Some(file_id) = input.avatar_uid {
        let scope = FileScope::PersonaAvatar {
            user_id: input.creator_id.clone(),
            persona_id: uid.clone(),
        };

        apply_avatar_uid_patch(deps, None, PatchField::Set(file_id), &scope)
            .await
            .map_err(|err| match err {
                ApplyAvatarPatchError::FileNotFound => CreatePersonaError::FileNotFound,
                ApplyAvatarPatchError::InvalidFileStatus => CreatePersonaError::InvalidFileStatus,
                ApplyAvatarPatchError::InvalidFileScope => CreatePersonaError::InvalidFileScope,
                ApplyAvatarPatchError::InternalError(message) => {
                    CreatePersonaError::InternalError(message)
                }
            })?
    } else {
        None
    };

    let persona = Persona {
        uid,
        creator_id: input.creator_id,
        name: input.name,
        description: input.description,
        avatar_uid,
    };

    let saved = persona::save_persona(&deps.db, persona)
        .await
        .map_err(|err| CreatePersonaError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(PersonaCreatedEvent {
            persona: saved.clone(),
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(saved)
}
