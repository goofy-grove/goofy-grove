use super::*;

#[test]
fn persona_created_event_is_constructible() {
    let event = PersonaCreatedEvent {
        persona: Persona {
            uid: PersonaId::try_new("persona-1".to_string()).unwrap(),
            creator_id: UserId::try_new("user-1".to_string()).unwrap(),
            name: PersonaName::try_new("Guide".to_string()).unwrap(),
            description: PersonaDescription::new("friendly".to_string()),
        },
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.persona.uid.inner(), "persona-1");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn persona_updated_event_is_constructible() {
    let event = PersonaUpdatedEvent {
        persona: Persona {
            uid: PersonaId::try_new("persona-1".to_string()).unwrap(),
            creator_id: UserId::try_new("user-1".to_string()).unwrap(),
            name: PersonaName::try_new("Guide".to_string()).unwrap(),
            description: PersonaDescription::new("updated".to_string()),
        },
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.persona.uid.inner(), "persona-1");
    assert_eq!(event.persona.description.inner(), "updated");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn persona_deleted_event_is_constructible() {
    let event = PersonaDeletedEvent {
        persona: Persona {
            uid: PersonaId::try_new("persona-1".to_string()).unwrap(),
            creator_id: UserId::try_new("user-1".to_string()).unwrap(),
            name: PersonaName::try_new("Guide".to_string()).unwrap(),
            description: PersonaDescription::new("deleted".to_string()),
        },
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.persona.uid.inner(), "persona-1");
    assert_eq!(event.persona.description.inner(), "deleted");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn character_created_event_is_constructible() {
    let event = CharacterCreatedEvent {
        character: Character {
            uid: CharacterId::try_new("character-1".to_string()).unwrap(),
            creator_id: UserId::try_new("user-1".to_string()).unwrap(),
            name: CharacterName::try_new("Knight".to_string()).unwrap(),
            description: CharacterDescription::new("brave".to_string()),
        },
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.character.uid.inner(), "character-1");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn character_updated_event_is_constructible() {
    let event = CharacterUpdatedEvent {
        character: Character {
            uid: CharacterId::try_new("character-1".to_string()).unwrap(),
            creator_id: UserId::try_new("user-1".to_string()).unwrap(),
            name: CharacterName::try_new("Knight".to_string()).unwrap(),
            description: CharacterDescription::new("updated".to_string()),
        },
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.character.uid.inner(), "character-1");
    assert_eq!(event.character.description.inner(), "updated");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn character_deleted_event_is_constructible() {
    let event = CharacterDeletedEvent {
        character: Character {
            uid: CharacterId::try_new("character-1".to_string()).unwrap(),
            creator_id: UserId::try_new("user-1".to_string()).unwrap(),
            name: CharacterName::try_new("Knight".to_string()).unwrap(),
            description: CharacterDescription::new("deleted".to_string()),
        },
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.character.uid.inner(), "character-1");
    assert_eq!(event.character.description.inner(), "deleted");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}
