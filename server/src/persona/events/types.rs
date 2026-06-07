use crate::{persona::db::persona::Persona, platform::events::Event};

#[derive(Debug, Clone)]
pub struct PersonaCreatedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<String>,
}

impl Event for PersonaCreatedEvent {}

#[derive(Debug, Clone)]
pub struct PersonaUpdatedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<String>,
}

impl Event for PersonaUpdatedEvent {}

#[derive(Debug, Clone)]
pub struct PersonaDeletedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<String>,
}

impl Event for PersonaDeletedEvent {}
