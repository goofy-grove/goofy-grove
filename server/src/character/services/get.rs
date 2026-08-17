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
    user_uid: &str,
) -> Result<Vec<Character>, GetCharactersError> {
    character::load_characters(&deps.db, user_uid)
        .await
        .map_err(|err| match err {
            character::LoadCharacterError::InternalError(message) => {
                GetCharactersError::InternalError(message)
            }
            // FIXME: remove one error type for different functions
            character::LoadCharacterError::NotFound => {
                GetCharactersError::InternalError("unexpected character not found".into())
            }
        })
}
