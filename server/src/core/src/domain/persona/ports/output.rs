use crate::domain::prelude::*;

pub trait LoadPersonasPort {
    fn load_personas(&self, user_id: &UserId) -> impl Future<Output = Vec<Persona>>;
}

#[derive(Debug, Clone)]
pub enum SavePersonaPortError {
    InternalError(String),
}

pub trait SavePersonaPort {
    fn save_persona(
        &self,
        persona: Persona,
    ) -> impl Future<Output = DomainResult<Persona, SavePersonaPortError>>;
}
