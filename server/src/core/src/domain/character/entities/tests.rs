use super::*;

#[test]
fn character_entity_exposes_fields() {
    let character = Character::new(
        CharacterId::try_new("character-1".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        CharacterName::try_new("Knight".to_string()).unwrap(),
        CharacterDescription::new("brave".to_string()),
    );

    assert_eq!(character.uid().inner(), "character-1");
    assert_eq!(character.creator_id().inner(), "user-1");
    assert_eq!(character.name().inner(), "Knight");
    assert_eq!(character.description().inner(), "brave");
}
