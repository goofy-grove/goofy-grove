use crate::domain::prelude::*;

pub trait Event: Send + Sync + 'static {}

pub struct PersonaCreatedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for PersonaCreatedEvent {}
