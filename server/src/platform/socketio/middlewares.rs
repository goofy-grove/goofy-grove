use std::{fmt::Display, sync::Arc};

use chrono::Utc;
use itertools::Itertools;
use keyv::Keyv;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use socketioxide::extract::{Data, SocketRef, State};
use tracing::info;

use crate::{
    auth::{AuthMiddlewareState, resolve_user_with_token_expiry},
    chat,
    platform::config::Config,
    user,
};

#[derive(Deserialize, Debug)]
pub struct AuthenticationPayload {
    pub token: String,
}

#[derive(Debug)]
pub enum AuthenticationError {
    Unauthorized,
    Unknown,
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationError::Unauthorized => write!(f, "Unauthorized"),
            AuthenticationError::Unknown => write!(f, "Unknown error"),
        }
    }
}

pub async fn authentication_middleware(
    socket: SocketRef,
    Data(data): Data<AuthenticationPayload>,
    State(config): State<Arc<Config>>,
    State(keyv): State<Arc<Keyv>>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AuthenticationError> {
    let auth_state = AuthMiddlewareState {
        config: config.clone(),
        db: db.clone(),
    };

    let username = keyv.get(socket.id.as_str()).await;

    if username.is_err() {
        info!(target: "application::socketio", err = ?username.err(), "Keyv error:");

        return Err(AuthenticationError::Unauthorized);
    }

    let username = username.unwrap();

    let username = if let Some(username) = username {
        username.to_string()
    } else {
        let (user, exp) = resolve_user_with_token_expiry(&auth_state, &data.token)
            .await
            .ok_or(AuthenticationError::Unauthorized)?;

        let ttl = (exp as i64 - Utc::now().timestamp()).max(0) as u64;

        info!(target: "application::socketio", socket_id = ?socket.id, username = ?user.name, ttl, "Token saved");

        let username_for_cache = user.name.clone();

        keyv.set_with_ttl(socket.id.as_str(), username_for_cache.clone(), ttl)
            .await
            .map_err(|err| {
                info!(target: "application::socketio", ?err, "Keyv error:");

                AuthenticationError::Unknown
            })?;

        socket.join(format!("user:{}", user.uid));

        username_for_cache
    };

    let user = user::get_by_name_db(&db, &username)
        .await
        .map_err(|_| AuthenticationError::Unknown)?;
    let user_chats = chat::get_user_chats(&db, &user.0.uid)
        .await
        .map_err(|_| AuthenticationError::Unknown)?
        .into_iter()
        .map(|chat| format!("chat:{}", chat.uid))
        .collect_vec();

    socket.extensions.insert(Arc::new(user));
    socket.join(socket.id);
    socket.join(user_chats);

    Ok(())
}
