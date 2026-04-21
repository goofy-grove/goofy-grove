use super::*;

#[test]
fn persona_name_trims_value() {
    assert_eq!(
        PersonaName::try_new("  Guide ".to_string())
            .unwrap()
            .inner(),
        "Guide"
    );
}

#[test]
fn persona_name_rejects_empty() {
    assert_eq!(
        PersonaName::try_new(" ".to_string()).unwrap_err(),
        PersonaNameValidationError::Empty
    );
}
