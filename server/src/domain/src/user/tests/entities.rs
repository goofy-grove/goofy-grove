use crate::prelude::*;

#[test]
fn test_user_valid() {
    let user = User::new(
        UserId::new("1".into()),
        UserName::new("John".into()),
        UserPassword::new("password".into()),
    );

    assert!(user.validate().is_ok());
}

#[test]
fn test_user_invalid_id() {
    let user = User::new(
        UserId::new("".into()),
        UserName::new("John".into()),
        UserPassword::new("password".into()),
    );

    let result = user.validate();

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        DomainValidationError::IdValidationError("user_id_is_empty".to_string())
    );
}

#[test]
fn test_user_invalid_name() {
    let user = User::new(
        UserId::new("1".into()),
        UserName::new("".into()),
        UserPassword::new("password".into()),
    );

    let result = user.validate();

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        DomainValidationError::NameValidationError("user_name_is_empty".to_string())
    );
}

#[test]
fn test_user_invalid_password() {
    let user = User::new(
        UserId::new("1".into()),
        UserName::new("John".into()),
        UserPassword::new("".into()),
    );

    let result = user.validate();

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        DomainValidationError::PasswordValidationError("user_password_is_empty".to_string())
    );
}
