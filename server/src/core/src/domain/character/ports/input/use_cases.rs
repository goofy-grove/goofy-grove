use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum CreateCharacterError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub trait CreateCharacterUseCase {
    fn create_character(
        &self,
        command: CreateCharacterCommand,
    ) -> impl Future<Output = Result<Character, CreateCharacterError>>;
}

#[derive(Debug, Clone, Error)]
pub enum UpdateCharacterError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub trait UpdateCharacterUseCase {
    fn update_character(
        &self,
        command: UpdateCharacterCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<Character, UpdateCharacterError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteCharacterError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait DeleteCharacterUseCase {
    fn delete_character(
        &self,
        command: DeleteCharacterCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), DeleteCharacterError>>;
}
