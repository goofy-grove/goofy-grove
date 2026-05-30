use super::*;

#[test]
fn user_entity_exposes_fields() {
    let user = User {
        uid: UserId::try_new("user-1".to_string()).unwrap(),
        name: Username::try_new("alice".to_string()).unwrap(),
        password: UserPassword::try_new("hashed".to_string()).unwrap(),
        avatar_uid: None,
    };

    assert_eq!(user.uid.inner(), "user-1");
    assert_eq!(user.name.inner(), "alice");
    assert_eq!(user.password.inner(), "hashed");
}
