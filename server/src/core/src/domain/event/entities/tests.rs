use super::*;

#[test]
fn persona_created_event_is_constructible() {
    let event = PersonaCreatedEvent {
        persona: Persona::new(
            PersonaId::try_new("persona-1".to_string()).unwrap(),
            UserId::try_new("user-1".to_string()).unwrap(),
            PersonaName::try_new("Guide".to_string()).unwrap(),
            PersonaDescription::new("friendly".to_string()),
        ),
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.persona.uid().inner(), "persona-1");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn persona_updated_event_is_constructible() {
    let event = PersonaUpdatedEvent {
        persona: Persona::new(
            PersonaId::try_new("persona-1".to_string()).unwrap(),
            UserId::try_new("user-1".to_string()).unwrap(),
            PersonaName::try_new("Guide".to_string()).unwrap(),
            PersonaDescription::new("updated".to_string()),
        ),
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.persona.uid().inner(), "persona-1");
    assert_eq!(event.persona.description().inner(), "updated");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}

#[test]
fn persona_deleted_event_is_constructible() {
    let event = PersonaDeletedEvent {
        persona: Persona::new(
            PersonaId::try_new("persona-1".to_string()).unwrap(),
            UserId::try_new("user-1".to_string()).unwrap(),
            PersonaName::try_new("Guide".to_string()).unwrap(),
            PersonaDescription::new("deleted".to_string()),
        ),
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    };

    assert_eq!(event.persona.uid().inner(), "persona-1");
    assert_eq!(event.persona.description().inner(), "deleted");
    assert_eq!(event.exclude_participants.len(), 1);
    assert_eq!(event.exclude_participants[0].inner(), "user-2");
}
