#![allow(unused)]

use thiserror::Error;

use crate::{
    app::AppDeps,
    chat::db::{self, Chat},
};

#[derive(Debug, Clone, Error)]
pub enum GetChatsError {
    #[error("Internal error: {0}")]
    Internal(String),
}

pub async fn get_chats(deps: &AppDeps, user_uid: &str) -> Result<Vec<Chat>, GetChatsError> {
    db::load_user_chats(&deps.db, user_uid)
        .await
        .map_err(|err| GetChatsError::Internal(err.to_string()))
}
