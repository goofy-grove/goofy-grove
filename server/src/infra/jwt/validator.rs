use std::sync::Arc;

use gg_core::domain::prelude::*;

use crate::infra::{config::Config, jwt::access_token::JwtAccessData};

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
    async fn validate_token(
        &self,
        first_token: &Token,
    ) -> DomainResult<TokenData, TokenValidatorPortError> {
        let validation = jsonwebtoken::decode::<JwtAccessData>(
            first_token.value(),
            &jsonwebtoken::DecodingKey::from_secret(self.config.jwt.access_token.secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| {
            if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                DomainError::ExternalServiceError(TokenValidatorPortError::TokenExpired)
            } else {
                DomainError::ExternalServiceError(TokenValidatorPortError::TokenInvalid)
            }
        })?;

        Ok(TokenData::new(validation.claims.sub))
    }
}
