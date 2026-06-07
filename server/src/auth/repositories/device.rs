use crate::infra::db::entities::{prelude::Tokens, tokens};
use chrono::DateTime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    SqlErr, TransactionTrait,
};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum LoadDeviceError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device not found")]
    DeviceNotFound,
}

#[derive(Debug, Clone, Error)]
pub enum InvalidateDeviceError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device not found")]
    DeviceNotFound,
}

#[derive(Debug, Clone, Error)]
pub enum SaveDeviceError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device already exists")]
    DeviceAlreadyExists,
}

#[derive(Debug, Clone)]
pub struct UserDevice {
    pub uid: String,
    pub hashed_token: String,
    pub user_id: String,
    pub user_agent: String,
    pub last_accessed_at: DateTime<chrono::Utc>,
}

impl From<tokens::Model> for UserDevice {
    fn from(value: tokens::Model) -> Self {
        Self {
            uid: value.uid,
            hashed_token: value.hashed_token,
            user_id: value.user_id,
            user_agent: value.user_agent,
            last_accessed_at: value.last_accessed_at.and_utc(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceRepository {
    connection: DatabaseConnection,
}

impl DeviceRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    async fn load_device(&self, hashed_token: &str) -> Result<UserDevice, LoadDeviceError> {
        let token = Tokens::find()
            .filter(tokens::Column::HashedToken.eq(hashed_token))
            .one(&self.connection)
            .await;

        match token {
            Ok(Some(token)) => Ok(token.into()),
            Ok(None) => Err(LoadDeviceError::DeviceNotFound),
            Err(err) => Err(LoadDeviceError::InternalError(err.to_string())),
        }
    }

    async fn create_device(&self, device: UserDevice) -> Result<UserDevice, SaveDeviceError> {
        let UserDevice {
            uid,
            hashed_token,
            user_id,
            user_agent,
            last_accessed_at,
        } = device;

        let token = tokens::ActiveModel {
            uid: Set(uid),
            hashed_token: Set(hashed_token),
            user_id: Set(user_id),
            user_agent: Set(user_agent),
            last_accessed_at: Set(last_accessed_at.naive_utc()),
        };

        let result = Tokens::insert(token)
            .exec_with_returning(&self.connection)
            .await;

        match result {
            Ok(token) => Ok(token.into()),
            Err(err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                    Err(SaveDeviceError::DeviceAlreadyExists)
                } else {
                    Err(SaveDeviceError::InternalError(err.to_string()))
                }
            }
        }
    }

    async fn invalidate_device(&self, hashed_token: &str) -> Result<(), InvalidateDeviceError> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|err| InvalidateDeviceError::InternalError(err.to_string()))?;

        let token = Tokens::find()
            .filter(tokens::Column::HashedToken.eq(hashed_token))
            .one(&tx)
            .await
            .map_err(|err| InvalidateDeviceError::InternalError(err.to_string()))?
            .ok_or(InvalidateDeviceError::DeviceNotFound)?;

        token
            .delete(&tx)
            .await
            .map_err(|err| InvalidateDeviceError::InternalError(err.to_string()))?;

        tx.commit()
            .await
            .map_err(|err| InvalidateDeviceError::InternalError(err.to_string()))?;

        Ok(())
    }
}
