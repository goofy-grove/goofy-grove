use thiserror::Error;

use crate::{
    app::AppDeps,
    auth::services::crypto,
    user::{self, User},
};

#[derive(Debug, Clone, Error)]
pub enum AuthenticateError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn authenticate(
    deps: &AppDeps,
    username: &str,
    password: &str,
) -> Result<User, AuthenticateError> {
    let (user, credentials) = user::get_by_name_db(&deps.db, username)
        .await
        .map_err(|_| AuthenticateError::InvalidCredentials)?;

    let verified = crypto::verify_password(password, &credentials.password_hash)
        .map_err(AuthenticateError::InternalError)?;

    if verified {
        Ok(user)
    } else {
        Err(AuthenticateError::InvalidCredentials)
    }
}
