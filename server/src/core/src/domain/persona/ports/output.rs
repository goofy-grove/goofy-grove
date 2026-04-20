use thiserror::Error;

use crate::domain::prelude::*;

pub trait LoadPersonasPort {
    fn load_personas(&self, user_id: &UserId) -> impl Future<Output = Vec<Persona>>;
}

#[derive(Debug, Clone, Error)]
pub enum SavePersonaPortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait SavePersonaPort {
    fn save_persona(
        &self,
        persona: Persona,
    ) -> impl Future<Output = Result<Persona, SavePersonaPortError>>;
}
