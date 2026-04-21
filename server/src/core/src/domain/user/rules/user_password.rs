use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum UserPasswordValidationError {
    #[error("user_password_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserPassword(String);
    error: UserPasswordValidationError;
    sanitize: |password: String| password.trim().to_owned();
    validate: |password: &str| {
        if password.is_empty() {
            Err(UserPasswordValidationError::Empty)
        } else {
            Ok(())
        }
    };
);

impl TryFrom<&str> for UserPassword {
    type Error = UserPasswordValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_owned())
    }
}
