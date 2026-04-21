use super::*;

#[test]
fn username_trims_value() {
    assert_eq!(
        Username::try_new("  alice ".to_string()).unwrap().inner(),
        "alice"
    );
}

#[test]
fn username_rejects_empty() {
    assert_eq!(
        Username::try_new(" ".to_string()).unwrap_err(),
        UsernameValidationError::Empty
    );
}
