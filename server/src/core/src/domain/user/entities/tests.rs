use super::*;

#[test]
fn user_entity_exposes_fields() {
    let user = User::new(
        UserId::try_new("user-1".to_string()).unwrap(),
        Username::try_new("alice".to_string()).unwrap(),
        UserPassword::try_new("hashed".to_string()).unwrap(),
    );

    assert_eq!(user.uid().inner(), "user-1");
    assert_eq!(user.name().inner(), "alice");
    assert_eq!(user.password().inner(), "hashed");
}
