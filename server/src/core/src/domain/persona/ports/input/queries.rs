use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum GetPersonasError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait GetPersonasQuery {
    fn get_personas(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<Persona>, GetPersonasError>>;
}
