use super::*;

#[test]
fn hashed_token_trims_value() {
    assert_eq!(
        HashedToken::try_new("  hash ".to_string()).unwrap().inner(),
        "hash"
    );
}

#[test]
fn hashed_token_rejects_empty() {
    assert_eq!(
        HashedToken::try_new(" ".to_string()).unwrap_err(),
        HashedTokenValidationError::Empty
    );
}
