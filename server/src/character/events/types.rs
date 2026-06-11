use crate::{character::db::character::Character, platform::events::Event};

#[derive(Debug, Clone)]
pub struct CharacterCreatedEvent {
    pub character: Character,
    pub exclude_participants: Vec<String>,
}

impl Event for CharacterCreatedEvent {}

#[derive(Debug, Clone)]
pub struct CharacterUpdatedEvent {
    pub character: Character,
    pub exclude_participants: Vec<String>,
}

impl Event for CharacterUpdatedEvent {}

#[derive(Debug, Clone)]
pub struct CharacterDeletedEvent {
    pub character_uid: String,
    pub creator_uid: String,
    pub exclude_participants: Vec<String>,
}

impl Event for CharacterDeletedEvent {}
