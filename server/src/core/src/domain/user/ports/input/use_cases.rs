use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum UpdateUserError {
    #[error("Not found")]
    NotFound,

    #[error("Access denied")]
    AccessDenied,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait UpdateUserUseCase {
    fn update_user(
        &self,
        command: UpdateUserCommand,
        user_id: UserId,
    ) -> impl Future<Output = Result<User, UpdateUserError>>;
}
