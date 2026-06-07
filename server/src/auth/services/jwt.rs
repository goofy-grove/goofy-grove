use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::platform::config::TokenConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub uid: String,
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct UserTokenInput {
    pub uid: String,
    pub username: String,
}

pub fn generate_token(
    user: UserTokenInput,
    config: &TokenConfig,
) -> Result<(String, usize), String> {
    let expires = (chrono::Utc::now() + chrono::Duration::seconds(config.expiration_time as i64))
        .timestamp() as usize;
    let UserTokenInput { uid, username } = user;
    let claims = TokenClaims {
        uid,
        sub: username,
        exp: expires,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
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

pub fn validate_token(
    token: &str,
    config: &TokenConfig,
) -> Result<TokenClaims, ValidateTokenError> {
    match jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(config.secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            Err(ValidateTokenError::TokenExpired)
        }
        Err(_) => Err(ValidateTokenError::TokenInvalid),
    }
}
