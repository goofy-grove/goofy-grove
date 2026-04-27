use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

pub trait Event: Send + Sync + 'static {}

pub struct PersonaCreatedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for PersonaCreatedEvent {}

pub struct PersonaUpdatedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for PersonaUpdatedEvent {}

pub struct PersonaDeletedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for PersonaDeletedEvent {}

pub struct CharacterCreatedEvent {
    pub character: Character,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for CharacterCreatedEvent {}

pub struct CharacterUpdatedEvent {
    pub character: Character,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for CharacterUpdatedEvent {}

pub struct CharacterDeletedEvent {
    pub character: Character,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for CharacterDeletedEvent {}
