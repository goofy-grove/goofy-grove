use domain::prelude::{DomainError, User, Validator};

#[test]
fn test_user_valid() {
    let user = User::new("1".to_string(), "John".to_string(), "password".to_string());

    assert!(user.validate().is_ok());
}

#[test]
fn test_user_invalid_id() {
    let user = User::new("".to_string(), "John".to_string(), "password".to_string());

    let result = user.validate();

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        DomainError::IdValidationError("user_id_is_empty".to_string())
    );
}

#[test]
fn test_user_invalid_name() {
    let user = User::new("1".to_string(), "".to_string(), "password".to_string());

    let result = user.validate();

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        DomainError::NameValidationError("user_name_is_empty".to_string())
    );
}

#[test]
fn test_user_invalid_password() {
    let user = User::new("1".to_string(), "John".to_string(), "".to_string());

    let result = user.validate();

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        DomainError::PasswordValidationError("user_password_is_empty".to_string())
    );
}
