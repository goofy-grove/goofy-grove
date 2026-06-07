use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::infra::config::TokenConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtData {
    pub uid: String,
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct UserData {
    pub uid: String,
    pub username: String,
}

async fn generate_token(user: UserData, config: &TokenConfig) -> Result<(String, usize), String> {
    let expires = (chrono::Utc::now() + chrono::Duration::seconds(config.expiration_time as i64))
        .timestamp() as usize;
    let UserData { uid, username } = user;
    let jwt_access_data = JwtData {
        uid,
        sub: username,
        exp: expires,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &jwt_access_data,
        &jsonwebtoken::EncodingKey::from_secret(config.secret.as_ref()),
    )
    .map_err(|err| err.to_string())
    .map(|token| (token, expires))
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidateTokenError {
    #[error("Token expired")]
    TokenExpired,

    #[error("Token invalid")]
    TokenInvalid,
}

async fn validate_token(token: &str, config: &TokenConfig) -> Result<JwtData, ValidateTokenError> {
    let validated_token = jsonwebtoken::decode::<JwtData>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(config.secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|token| token.claims)
    .map_err(|err| {
        if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
            ValidateTokenError::TokenExpired
        } else {
            ValidateTokenError::TokenInvalid
        }
    })?;

    Ok(validated_token)
}
