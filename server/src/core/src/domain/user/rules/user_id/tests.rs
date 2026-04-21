use super::*;

#[test]
fn user_id_trims_value() {
    assert_eq!(
        UserId::try_new("  id-1 ".to_string()).unwrap().inner(),
        "id-1"
    );
}

#[test]
fn user_id_rejects_empty() {
    assert_eq!(
        UserId::try_new(" ".to_string()).unwrap_err(),
        UserIdValidationError::Empty
    );
}
