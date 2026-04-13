use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum CreatePersonaError {
    InternalError(String),
}

pub trait CreatePersonaUseCase {
    fn create_persona(
        &self,
        command: CreatePersonaCommand,
    ) -> impl Future<Output = DomainResult<Persona, CreatePersonaError>>;
}
