use std::sync::Arc;

use gg_core::domain::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infra::config::Config;

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtRefreshData {
    pub uid: String,
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtRefreshTokenGenerator {
    config: Arc<Config>,
}

impl JwtRefreshTokenGenerator {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl TokenGeneratorPort for JwtRefreshTokenGenerator {
    async fn generate_token(
        &self,
        user: &User,
    ) -> DomainResult<(String, usize), TokenGeneratorPortError> {
        let expires = (chrono::Utc::now()
            + chrono::Duration::seconds(self.config.jwt.refresh_token.expiration_time as i64))
        .timestamp() as usize;
        let jwt_access_data = JwtRefreshData {
            uid: user.uid().value().to_owned(),
            sub: user.name().value().to_owned(),
            exp: expires,
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &jwt_access_data,
            &jsonwebtoken::EncodingKey::from_secret(self.config.jwt.refresh_token.secret.as_ref()),
        )
        .map_err(|err| {
            DomainError::ExternalServiceError(TokenGeneratorPortError::InternalError(
                err.to_string(),
            ))
        })?;

        Ok((
            token,
            self.config.jwt.refresh_token.expiration_time as usize,
        ))
    }
}
