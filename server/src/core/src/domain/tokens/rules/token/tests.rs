use super::*;

#[test]
fn token_trims_value() {
    assert_eq!(Token::try_new("  raw ".to_string()).unwrap().inner(), "raw");
}

#[test]
fn token_rejects_empty() {
    assert_eq!(
        Token::try_new(" ".to_string()).unwrap_err(),
        TokenValidationError::Empty
    );
}
