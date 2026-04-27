use super::*;

#[test]
fn character_description_trims_value() {
    assert_eq!(
        CharacterDescription::new("  brave ".to_string()).inner(),
        "brave"
    );
}
