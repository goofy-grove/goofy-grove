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

        Ok(UserToken {
            uid: TokenId::try_new(token.uid)
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
            hashed_token: HashedToken::try_new(token.hashed_token)
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
            user_id: UserId::try_new(token.user_id)
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
            user_agent: UserAgent::new(token.user_agent),
            last_accessed_at: LastAccessedAt::try_new(token.last_accessed_at.and_utc().timestamp())
                .map_err(|err| LoadDevicePortError::InternalError(err.to_string()))?,
        })
    }
}

impl SaveDevicePort for TokensRepository {
    async fn create_device(&self, token: UserToken) -> Result<UserToken, SaveDevicePortError> {
        let UserToken {
            uid,
            hashed_token,
            user_id,
            user_agent,
            last_accessed_at,
        } = token;

        let token = tokens::ActiveModel {
            uid: Set(uid.into_inner()),
            hashed_token: Set(hashed_token.into_inner()),
            user_id: Set(user_id.into_inner()),
            user_agent: Set(user_agent.into_inner()),
            last_accessed_at: Set(DateTime::from_timestamp(last_accessed_at.into_inner(), 0)
                .unwrap()
                .naive_utc()),
        };

        let result = Tokens::insert(token)
            .exec_with_returning(&self.connection)
            .await;

        match result {
            Ok(token) => Ok(UserToken {
                uid: TokenId::try_new(token.uid)
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
                hashed_token: HashedToken::try_new(token.hashed_token)
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
                user_id: UserId::try_new(token.user_id)
                    .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
                user_agent: UserAgent::new(token.user_agent),
                last_accessed_at: LastAccessedAt::try_new(
                    token.last_accessed_at.and_utc().timestamp(),
                )
                .map_err(|err| SaveDevicePortError::InternalError(err.to_string()))?,
            }),
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
