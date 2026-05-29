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

#[derive(Debug, Clone, Error)]
pub enum UpdatePersonaError {
    #[error("Not found")]
    NotFound,

    #[error("Access denied")]
    AccessDenied,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub trait UpdatePersonaUseCase {
    fn update_persona(
        &self,
        command: UpdatePersonaCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<Persona, UpdatePersonaError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeletePersonaError {
    #[error("Not found")]
    NotFound,

    #[error("Access denied")]
    AccessDenied,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait DeletePersonaUseCase {
    fn delete_persona(
        &self,
        command: DeletePersonaCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), DeletePersonaError>>;
}
