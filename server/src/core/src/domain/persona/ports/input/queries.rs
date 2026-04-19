use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum GetPersonasErorr {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait GetPersonasQuery {
    fn get_personas(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = DomainResult<Vec<Persona>, GetPersonasErorr>>;
}
