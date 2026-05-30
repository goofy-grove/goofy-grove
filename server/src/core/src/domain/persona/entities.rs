use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Persona {
    pub uid: PersonaId,
    pub creator_id: UserId,
    pub name: PersonaName,
    pub description: PersonaDescription,
    pub avatar_uid: Option<FileId>,
}
