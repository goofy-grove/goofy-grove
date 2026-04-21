use super::*;

#[test]
fn user_password_trims_value() {
    assert_eq!(
        UserPassword::try_new("  hash ".to_string())
            .unwrap()
            .inner(),
        "hash"
    );
}

#[test]
fn user_password_rejects_empty() {
    assert_eq!(
        UserPassword::try_new(" ".to_string()).unwrap_err(),
        UserPasswordValidationError::Empty
    );
}
