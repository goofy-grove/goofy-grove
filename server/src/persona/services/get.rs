use thiserror::Error;

use crate::{
    app::AppDeps,
    persona::db::persona::{self, Persona},
};

#[derive(Debug, Clone, Error)]
pub enum GetPersonasError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn get_personas(deps: &AppDeps, user_id: &str) -> Result<Vec<Persona>, GetPersonasError> {
    persona::load_personas(&deps.db, user_id)
        .await
        .map_err(|err| match err {
            persona::LoadPersonaError::NotFound => {
                GetPersonasError::InternalError("Persona not found".into())
            }
            persona::LoadPersonaError::InternalError(message) => {
                GetPersonasError::InternalError(message)
            }
        })
}
