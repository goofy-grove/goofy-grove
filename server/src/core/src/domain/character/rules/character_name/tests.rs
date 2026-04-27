use super::*;

#[test]
fn character_name_trims_value() {
    assert_eq!(
        CharacterName::try_new("  Knight ".to_string())
            .unwrap()
            .inner(),
        "Knight"
    );
}

#[test]
fn character_name_rejects_empty() {
    assert_eq!(
        CharacterName::try_new(" ".to_string()).unwrap_err(),
        CharacterNameValidationError::Empty
    );
}
