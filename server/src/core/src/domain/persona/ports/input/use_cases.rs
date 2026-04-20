use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum CreatePersonaError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub trait CreatePersonaUseCase {
    fn create_persona(
        &self,
        command: CreatePersonaCommand,
    ) -> impl Future<Output = Result<Persona, CreatePersonaError>>;
}
