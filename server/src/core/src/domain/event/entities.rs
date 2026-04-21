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
