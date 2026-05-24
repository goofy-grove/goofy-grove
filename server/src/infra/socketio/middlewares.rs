use std::{fmt::Display, sync::Arc};

use chrono::Utc;
use gg_core::{
    application::user::GetUserByNameService,
    domain::prelude::{GetUserByNameQuery, Token, TokenData, TokenValidatorPort, Username},
};
use keyv::Keyv;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use socketioxide::extract::{Data, SocketRef, State};
use tracing::info;

use crate::infra::{config::Config, db::UserRepository, jwt::JwtAccessTokenValidator};

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

// FIXME: separate function
pub async fn authentication_middleware(
    socket: SocketRef,
    Data(data): Data<AuthenticationPayload>,
    State(config): State<Arc<Config>>,
    State(keyv): State<Arc<Keyv>>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AuthenticationError> {
    let jwt_token_validator = JwtAccessTokenValidator::new(config.clone());
    let user_get_service = GetUserByNameService::new(UserRepository::new(db));

    let username = keyv.get(socket.id.as_str()).await;

    if username.is_err() {
        info!(target: "application::socketio", err = ?username.err(), "Keyv error:");

        return Err(AuthenticationError::Unauthorized);
    }

    let username = username.unwrap();

    let username = if let Some(username) = username {
        username.to_string()
    } else {
        let token = Token::try_new(data.token).map_err(|_| AuthenticationError::Unauthorized)?;
        let token_data = jwt_token_validator.validate_token(&token).await;

        if token_data.is_err() {
            info!(target: "application::socketio", err = ?token_data.err(), "Token validation error:");

            keyv.remove(socket.id.as_str())
                .await
                .map_err(|err| {
                    info!(target: "application::socketio", ?err, "Keyv error:");
                })
                .unwrap();

            return Err(AuthenticationError::Unauthorized);
        }

        let TokenData {
            uid,
            username,
            expires_at,
        } = token_data.unwrap();

        let ttl = (*expires_at.inner() as i64 - Utc::now().timestamp()).max(0) as u64;

        info!(target: "application::socketio", socket_id = ?socket.id, username = ?username, ttl, "Token saved");

        let username_for_cache = username.clone().into_inner();

        keyv.set_with_ttl(socket.id.as_str(), username_for_cache, ttl)
            .await
            .map_err(|err| {
                info!(target: "application::socketio", ?err, "Keyv error:");

                AuthenticationError::Unknown
            })?;

        socket.join(format!("user:{}", uid.into_inner()));

        username.into_inner()
    };

    let username = Username::try_new(username).map_err(|_| AuthenticationError::Unauthorized)?;
    let user = user_get_service.get_user_by_name(&username).await;

    if user.is_err() {
        info!(target: "application::socketio", err = ?user.err(), "User getting error:");

        return Err(AuthenticationError::Unknown);
    }

    let user = Arc::new(user.unwrap());

    socket.extensions.insert(user);
    socket.join(socket.id);

    Ok(())
}
