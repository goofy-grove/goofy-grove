use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct CreatePersonaCommand {
    pub name: PersonaName,
    pub creator_id: UserId,
    pub description: PersonaDescription,
    pub exclude_participants: Vec<ParticipantId>,
}

#[derive(Debug, Clone)]
pub struct UpdatePersonaCommand {
    pub id: PersonaId,
    pub name: Option<PersonaName>,
    pub description: Option<PersonaDescription>,
    pub exclude_participants: Vec<ParticipantId>,
}

#[derive(Debug, Clone)]
pub struct DeletePersonaCommand {
    pub id: PersonaId,
    pub exclude_participants: Vec<ParticipantId>,
}
