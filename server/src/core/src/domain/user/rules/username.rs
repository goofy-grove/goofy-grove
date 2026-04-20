use thiserror::Error;

use crate::impl_new_type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum UsernameValidationError {
    #[error("username_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Username(String);
    error: UsernameValidationError;
    sanitize: |username: String| username.trim().to_owned();
    validate: |username: &str| {
        if username.is_empty() {
            Err(UsernameValidationError::Empty)
        } else {
            Ok(())
        }
    };
);

impl TryFrom<&str> for Username {
    type Error = UsernameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_owned())
    }
}
