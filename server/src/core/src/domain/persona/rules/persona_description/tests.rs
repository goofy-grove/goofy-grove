use super::*;

#[test]
fn persona_description_trims_value() {
    assert_eq!(
        PersonaDescription::new("  friendly ".to_string()).inner(),
        "friendly"
    );
}
