use thiserror::Error;

use crate::domain::prelude::*;

pub trait LoadCharactersPort {
    fn load_characters(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<Character>, LoadCharactersPortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum LoadCharactersPortError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait LoadCharacterPort {
    fn load_character(
        &self,
        character_id: &CharacterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Character, LoadCharactersPortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum SaveCharacterPortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait SaveCharacterPort {
    fn save_character(
        &self,
        character: Character,
    ) -> impl Future<Output = Result<Character, SaveCharacterPortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteCharacterPortError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait DeleteCharacterPort {
    fn delete_character(
        &self,
        character_id: &CharacterId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), DeleteCharacterPortError>>;
}
