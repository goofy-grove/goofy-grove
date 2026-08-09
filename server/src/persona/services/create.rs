use thiserror::Error;

use crate::{
    app::AppDeps,
    persona::{
        db::persona::{self, Persona},
        events::types::PersonaCreatedEvent,
    },
    platform::events::EventPublisher,
    platform::util,
};

#[derive(Debug, Clone, Error)]
pub enum CreatePersonaError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct CreatePersonaInput {
    pub name: String,
    pub description: String,
    pub creator_uid: String,
    pub exclude_participants: Vec<String>,
}

pub async fn create_persona(
    deps: &AppDeps,
    input: CreatePersonaInput,
) -> Result<Persona, CreatePersonaError> {
    let persona = Persona {
        uid: util::uid_generator::generate_uid("persona"),
        creator_uid: input.creator_uid,
        name: input.name,
        description: input.description,
        avatar_uid: None,
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
