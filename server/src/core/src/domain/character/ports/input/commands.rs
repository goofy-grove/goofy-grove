use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateCharacterCommand {
    pub name: CharacterName,
    pub creator_id: UserId,
    pub description: CharacterDescription,
    pub exclude_participants: Vec<ParticipantId>,
}

#[derive(Debug, Clone)]
pub struct UpdateCharacterCommand {
    pub id: CharacterId,
    pub name: Option<CharacterName>,
    pub description: Option<CharacterDescription>,
    pub exclude_participants: Vec<ParticipantId>,
}

#[derive(Debug, Clone)]
pub struct DeleteCharacterCommand {
    pub id: CharacterId,
    pub exclude_participants: Vec<ParticipantId>,
}
