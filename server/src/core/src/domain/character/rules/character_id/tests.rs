use super::*;

#[test]
fn character_id_trims_value() {
    assert_eq!(
        CharacterId::try_new("  c-1 ".to_string()).unwrap().inner(),
        "c-1"
    );
}

#[test]
fn character_id_rejects_empty() {
    assert_eq!(
        CharacterId::try_new(" ".to_string()).unwrap_err(),
        CharacterIdValidationError::Empty
    );
}
