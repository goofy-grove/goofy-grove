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
    ) -> Result<UserToken, LoadDevicePortError> {
        let token = Tokens::find()
            .filter(tokens::Column::HashedToken.eq(hashed_token.inner()))
            .one(&self.connection)
            .await
            .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?
            .ok_or(LoadDevicePortError::DeviceNotFound)?;

        Ok(UserToken::new(
            TokenId::try_new(token.uid)
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
            HashedToken::try_new(token.hashed_token)
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
            UserId::try_new(token.user_id)
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
            UserAgent::new(token.user_agent),
            LastAccessedAt::try_new(token.last_accessed_at.and_utc().timestamp())
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
        ))
    }
}

impl SaveDevicePort for TokensRepository {
    async fn create_device(&self, token: UserToken) -> Result<UserToken, SaveDevicePortError> {
        let token = tokens::ActiveModel {
            uid: Set(token.uid().inner().to_owned()),
            hashed_token: Set(token.hashed_token().inner().to_owned()),
            user_id: Set(token.user_id().inner().to_owned()),
            user_agent: Set(token.user_agent().inner().to_owned()),
            last_accessed_at: Set(DateTime::from_timestamp(
                token.last_accessed_at().inner().to_owned(),
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
                TokenId::try_new(token.uid)
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
                HashedToken::try_new(token.hashed_token)
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
                UserId::try_new(token.user_id)
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
                UserAgent::new(token.user_agent),
                LastAccessedAt::try_new(token.last_accessed_at.and_utc().timestamp())
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
            )),
            Err(err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                    Err(SaveDevicePortError::DeviceAlreadyExists)
                } else {
                    Err(SaveDevicePortError::InternalError(err.to_string()))
                }
            }
        }
    }
}

impl InvalidateDevicePort for TokensRepository {
    async fn invalidate_device(
        &self,
        hashed_token: &HashedToken,
    ) -> Result<(), InvalidateDevicePortError> {
        let token = Tokens::find()
            .filter(tokens::Column::HashedToken.eq(hashed_token.inner()))
            .one(&self.connection)
            .await
            .map_err(|err| InvalidateDevicePortError::InternalError(err.to_string()))?
            .ok_or(InvalidateDevicePortError::DeviceNotFound)?;

        token
            .delete(&self.connection)
            .await
            .map_err(|err| InvalidateDevicePortError::InternalError(err.to_string()))?;

        Ok(())
    }
}
