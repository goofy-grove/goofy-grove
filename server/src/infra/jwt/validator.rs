use std::sync::Arc;

use gg_core::domain::prelude::*;

use crate::infra::{
    config::Config,
    jwt::{access_token::JwtAccessData, refresh_token::JwtRefreshData},
};

#[derive(Debug, Clone)]
pub struct JwtAccessTokenValidator {
    config: Arc<Config>,
}

impl JwtAccessTokenValidator {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl TokenValidatorPort for JwtAccessTokenValidator {
    async fn validate_token(&self, token: &Token) -> Result<TokenData, TokenValidatorPortError> {
        let validation = jsonwebtoken::decode::<JwtAccessData>(
            token.inner(),
            &jsonwebtoken::DecodingKey::from_secret(self.config.jwt.access_token.secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| {
            if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                TokenValidatorPortError::TokenExpired
            } else {
                TokenValidatorPortError::TokenInvalid
            }
        })?;

        Ok(TokenData {
            uid: UserId::try_new(validation.claims.uid)
                .map_err(|err| TokenValidatorPortError::InternalError(err.to_string()))?,
            username: Username::try_new(validation.claims.sub)
                .map_err(|err| TokenValidatorPortError::InternalError(err.to_string()))?,
            expires_at: TokenExpires::new(validation.claims.exp),
        })
    }
}

#[derive(Debug, Clone)]
pub struct JwtRefreshTokenValidator {
    config: Arc<Config>,
}

impl JwtRefreshTokenValidator {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl TokenValidatorPort for JwtRefreshTokenValidator {
    async fn validate_token(&self, token: &Token) -> Result<TokenData, TokenValidatorPortError> {
        let validation = jsonwebtoken::decode::<JwtRefreshData>(
            token.inner(),
            &jsonwebtoken::DecodingKey::from_secret(self.config.jwt.refresh_token.secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| {
            if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                TokenValidatorPortError::TokenExpired
            } else {
                TokenValidatorPortError::TokenInvalid
            }
        })?;

        Ok(TokenData {
            uid: UserId::try_new(validation.claims.uid)
                .map_err(|err| TokenValidatorPortError::InternalError(err.to_string()))?,
            username: Username::try_new(validation.claims.sub)
                .map_err(|err| TokenValidatorPortError::InternalError(err.to_string()))?,
            expires_at: TokenExpires::new(validation.claims.exp),
        })
    }
}
