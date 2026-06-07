use thiserror::Error;

use crate::{
    app::AppDeps,
    file::public::{OrphanAvatarError, orphan_avatar_if_present},
    persona::{
        db::persona::{self},
        events::types::PersonaDeletedEvent,
    },
    platform::events::EventPublisher,
};

#[derive(Debug, Clone, Error)]
pub enum DeletePersonaError {
    #[error("Not found")]
    NotFound,

    #[error("Access denied")]
    AccessDenied,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct DeletePersonaInput {
    pub id: String,
    pub user_id: String,
    pub exclude_participants: Vec<String>,
}

pub async fn delete_persona(
    deps: &AppDeps,
    input: DeletePersonaInput,
) -> Result<(), DeletePersonaError> {
    let persona = persona::load_persona(&deps.db, &input.id, &input.user_id)
        .await
        .map_err(|err| match err {
            persona::LoadPersonaError::NotFound => DeletePersonaError::NotFound,
            persona::LoadPersonaError::InternalError(message) => {
                DeletePersonaError::InternalError(message)
            }
        })?;

    if persona.creator_id != input.user_id {
        return Err(DeletePersonaError::AccessDenied);
    }

    if let Err(OrphanAvatarError::InternalError(message)) =
        orphan_avatar_if_present(deps, persona.avatar_uid.clone()).await
    {
        return Err(DeletePersonaError::InternalError(message));
    }

    persona::delete_persona(&deps.db, &input.id, &input.user_id)
        .await
        .map_err(|err| match err {
            persona::DeletePersonaError::NotFound => DeletePersonaError::NotFound,
            persona::DeletePersonaError::InternalError(message) => {
                DeletePersonaError::InternalError(message)
            }
        })?;

    deps.event_bus
        .publish(PersonaDeletedEvent {
            persona,
            exclude_participants: input.exclude_participants,
        })
        .await;

    Ok(())
}
