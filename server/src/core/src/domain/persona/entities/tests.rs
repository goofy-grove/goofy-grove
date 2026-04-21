use super::*;

#[test]
fn persona_entity_exposes_fields() {
    let persona = Persona::new(
        PersonaId::try_new("persona-1".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        PersonaName::try_new("Guide".to_string()).unwrap(),
        PersonaDescription::new("friendly".to_string()),
    );

    assert_eq!(persona.uid().inner(), "persona-1");
    assert_eq!(persona.creator_id().inner(), "user-1");
    assert_eq!(persona.name().inner(), "Guide");
    assert_eq!(persona.description().inner(), "friendly");
}
