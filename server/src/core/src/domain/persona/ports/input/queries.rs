use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum GetPersonasErorr {
    InternalError(String),
}

pub trait GetPersonasQuery {
    fn get_personas(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = DomainResult<Vec<Persona>, GetPersonasErorr>>;
}
