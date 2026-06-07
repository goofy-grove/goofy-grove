use thiserror::Error;

use crate::{
    app::AppDeps,
    auth::services::crypto::hash_password,
    platform::util,
    user::db::user::{LoadUserError, User, save_user},
};

#[derive(Debug, Clone, Error)]
pub enum RegisterError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Failed to hash password: {0}")]
    Hash(String),

    #[error("Failed to save user: {0}")]
    Save(#[from] LoadUserError),
}

pub async fn register(deps: &AppDeps, name: &str, password: &str) -> Result<User, RegisterError> {
    if name.is_empty() {
        return Err(RegisterError::Validation("Name is empty".into()));
    }
    if password.is_empty() {
        return Err(RegisterError::Validation("Password is empty".into()));
    }

    let hashed = hash_password(password).map_err(RegisterError::Hash)?;

    let user = User {
        uid: util::id_generator::generate_id("user"),
        name: name.to_owned(),
        password: hashed,
        avatar_uid: None,
    };

    save_user(&deps.db, user)
        .await
        .map_err(RegisterError::Save)
}
