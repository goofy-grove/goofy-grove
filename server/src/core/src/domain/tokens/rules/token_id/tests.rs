use super::*;

#[test]
fn token_id_trims_value() {
    assert_eq!(
        TokenId::try_new("  t-1 ".to_string()).unwrap().inner(),
        "t-1"
    );
}

#[test]
fn token_id_rejects_empty() {
    assert_eq!(
        TokenId::try_new(" ".to_string()).unwrap_err(),
        TokenIdValidationError::Empty
    );
}
