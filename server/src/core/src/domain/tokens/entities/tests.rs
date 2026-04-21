use super::*;

#[test]
fn token_data_exposes_fields() {
    let data = TokenData::new(
        UserId::try_new("user-1".to_string()).unwrap(),
        Username::try_new("alice".to_string()).unwrap(),
        TokenExpires::new(60),
    );

    assert_eq!(data.uid().inner(), "user-1");
    assert_eq!(data.username().inner(), "alice");
    assert_eq!(data.expires_at().inner(), &60);
}

#[test]
fn user_token_exposes_fields() {
    let token = UserToken::new(
        TokenId::try_new("token-1".to_string()).unwrap(),
        HashedToken::try_new("hashed".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        UserAgent::new("browser".to_string()),
        LastAccessedAt::try_new(10).unwrap(),
    );

    assert_eq!(token.uid().inner(), "token-1");
    assert_eq!(token.hashed_token().inner(), "hashed");
    assert_eq!(token.user_id().inner(), "user-1");
    assert_eq!(token.user_agent().inner(), "browser");
    assert_eq!(token.last_accessed_at().inner(), &10);
}
