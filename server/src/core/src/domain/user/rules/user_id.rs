use thiserror::Error;

use crate::impl_new_type;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum UserIdValidationError {
    #[error("user_id_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserId(String);
    error: UserIdValidationError;
    sanitize: |id: String| id.trim().to_owned();
    validate: |id: &str| {
        if id.is_empty() {
            Err(UserIdValidationError::Empty)
        } else {
            Ok(())
        }
    };
);

impl TryFrom<&str> for UserId {
    type Error = UserIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_owned())
    }
}
