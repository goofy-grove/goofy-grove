use crate::{domain::prelude::Persona, impl_as_domain_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticipantId(String);

impl_as_domain_newtype!(ParticipantId -> String);

pub trait Event: Send + Sync + 'static {}

pub struct PersonaCreatedEvent {
    pub persona: Persona,
    pub exclude_participants: Vec<ParticipantId>,
}

impl Event for PersonaCreatedEvent {}
