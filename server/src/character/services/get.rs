use thiserror::Error;

use crate::{
    app::AppDeps,
    character::db::character::{self, Character},
};

#[derive(Debug, Clone, Error)]
pub enum GetCharactersError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn get_characters(
    deps: &AppDeps,
    user_id: &str,
) -> Result<Vec<Character>, GetCharactersError> {
    character::load_characters(&deps.db, user_id)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::NotFound => {
                GetCharactersError::InternalError("Character not found".into())
            }
            character::LoadCharacterError::InternalError(message) => {
                GetCharactersError::InternalError(message)
            }
        })
}
