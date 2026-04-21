use super::*;

#[test]
fn persona_id_trims_value() {
    assert_eq!(
        PersonaId::try_new("  p-1 ".to_string()).unwrap().inner(),
        "p-1"
    );
}

#[test]
fn persona_id_rejects_empty() {
    assert_eq!(
        PersonaId::try_new(" ".to_string()).unwrap_err(),
        PersonaIdValidationError::Empty
    );
}
