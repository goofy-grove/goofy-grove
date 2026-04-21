use super::*;

#[test]
fn secret_trims_value() {
    assert_eq!(
        Secret::try_new("  value  ".to_string()).unwrap().inner(),
        "value"
    );
}

#[test]
fn secret_rejects_empty() {
    assert_eq!(
        Secret::try_new("   ".to_string()).unwrap_err(),
        SecretValidationError::Empty
    );
}
