use crate::infra::db::entities::{prelude::Tokens, tokens};
use chrono::DateTime;
use gg_core::domain::prelude::*;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, SqlErr,
};

#[derive(Debug, Clone)]
pub struct TokensRepository {
    connection: DatabaseConnection,
}

impl TokensRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl LoadDevicePort for TokensRepository {
    async fn load_device(
        &self,
        hashed_token: &HashedToken,
    ) -> DomainResult<UserToken, LoadDevicePortError> {
        let token = Tokens::find()
            .filter(tokens::Column::HashedToken.eq(hashed_token.value()))
            .one(&self.connection)
            .await
            .map_err(|err| {
                DomainError::ExternalServiceError(LoadDevicePortError::InternalError(
                    err.to_string(),
                ))
            })?
            .ok_or(DomainError::ExternalServiceError(
                LoadDevicePortError::DeviceNotFound,
            ))?;

        Ok(UserToken::new(
            TokenId::new(token.uid),
            HashedToken::new(token.hashed_token),
            UserId::new(token.user_id),
            UserAgent::new(token.user_agent),
            LastAccessedAt::new(token.last_accessed_at.and_utc().timestamp()),
        ))
    }
}

impl SaveDevicePort for TokensRepository {
    async fn create_device(
        &self,
        token: UserToken,
    ) -> DomainResult<UserToken, SaveDevicePortError> {
        let token = tokens::ActiveModel {
            uid: Set(token.uid().value().to_owned()),
            hashed_token: Set(token.hashed_token().value().to_owned()),
            user_id: Set(token.user_id().value().to_owned()),
            user_agent: Set(token.user_agent().value().to_owned()),
            last_accessed_at: Set(DateTime::from_timestamp(
                token.last_accessed_at().value().to_owned(),
                0,
            )
            .unwrap()
            .naive_utc()),
        };

        let result = Tokens::insert(token)
            .exec_with_returning(&self.connection)
            .await;

        match result {
            Ok(token) => Ok(UserToken::new(
                TokenId::new(token.uid),
                HashedToken::new(token.hashed_token),
                UserId::new(token.user_id),
                UserAgent::new(token.user_agent),
                LastAccessedAt::new(token.last_accessed_at.and_utc().timestamp()),
            )),
            Err(err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                    Err(DomainError::ExternalServiceError(
                        SaveDevicePortError::DeviceAlreadyExists,
                    ))
                } else {
                    Err(DomainError::ExternalServiceError(
                        SaveDevicePortError::InternalError(err.to_string()),
                    ))
                }
            }
        }
    }
}

impl InvalidateDevicePort for TokensRepository {
    async fn invalidate_device(
        &self,
        hashed_token: &HashedToken,
    ) -> DomainResult<(), InvalidateDevicePortError> {
        let token = Tokens::find()
            .filter(tokens::Column::HashedToken.eq(hashed_token.value()))
            .one(&self.connection)
            .await
            .map_err(|err| {
                DomainError::ExternalServiceError(InvalidateDevicePortError::InternalError(
                    err.to_string(),
                ))
            })?
            .ok_or(DomainError::ExternalServiceError(
                InvalidateDevicePortError::DeviceNotFound,
            ))?;

        token.delete(&self.connection).await.map_err(|err| {
            DomainError::ExternalServiceError(InvalidateDevicePortError::InternalError(
                err.to_string(),
            ))
        })?;

        Ok(())
    }
}
