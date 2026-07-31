use thiserror::Error;

use crate::{
    app::AppDeps,
    auth::services::crypto::hash_password,
    platform::util,
    user::db::user::{LoadUserError, User, UserCredentials, save_user},
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
    let uid = util::uid_generator::generate_uid("user");

    let user = User {
        uid: uid.clone(),
        name: name.to_owned(),
        avatar_uid: None,
    };
    let credentials = UserCredentials {
        user_uid: uid,
        password_hash: hashed,
    };

    save_user(&deps.db, user, credentials)
        .await
        .map_err(RegisterError::Save)
}
