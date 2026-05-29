use thiserror::Error;

use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PersonaAccessError {
    #[error("Access denied")]
    AccessDenied,
}

pub fn can_update_persona(actor: &UserId, persona: &Persona) -> Result<(), PersonaAccessError> {
    can_modify_persona(actor, persona)
}

pub fn can_delete_persona(actor: &UserId, persona: &Persona) -> Result<(), PersonaAccessError> {
    can_modify_persona(actor, persona)
}

fn can_modify_persona(actor: &UserId, persona: &Persona) -> Result<(), PersonaAccessError> {
    if &persona.creator_id != actor {
        return Err(PersonaAccessError::AccessDenied);
    }

    Ok(())
}
