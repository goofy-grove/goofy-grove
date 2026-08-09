use thiserror::Error;

use crate::{
    app::AppDeps,
    persona::{
        db::persona::{self, Persona},
        events::types::PersonaUpdatedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum UpdatePersonaError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct UpdatePersonaInput {
    pub persona_uid: String,
    pub user_uid: String,
    pub name: Option<String>,
    pub description: Option<String>,
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

    let updated = Persona {
        uid: persona.uid,
        creator_uid: persona.creator_uid,
        name: input.name.unwrap_or(persona.name),
        description: input.description.unwrap_or(persona.description),
        avatar_uid: persona.avatar_uid,
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
