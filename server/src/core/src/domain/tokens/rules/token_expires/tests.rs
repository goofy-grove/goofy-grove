use super::*;

#[test]
fn token_expires_keeps_value() {
    assert_eq!(TokenExpires::new(60).inner(), &60);
}
