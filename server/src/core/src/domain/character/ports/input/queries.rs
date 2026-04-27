use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum GetCharactersError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait GetCharactersQuery {
    fn get_characters(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<Character>, GetCharactersError>>;
}
