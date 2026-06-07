use chrono::Utc;
use thiserror::Error;

use crate::{
    app::AppDeps,
    auth::{
        db::device::{
            InvalidateDeviceError, SaveDeviceError, UserDevice, create_device, invalidate_device,
        },
        services::crypto,
    },
    platform::util,
};

#[derive(Debug, Clone, Error)]
pub enum RegisterDeviceError {
    #[error("Device already exists")]
    DeviceAlreadyExists,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct RegisterDeviceInput {
    pub refresh_token: String,
    pub user_id: String,
    pub user_agent: String,
}

pub async fn register_device(
    deps: &AppDeps,
    input: RegisterDeviceInput,
) -> Result<(), RegisterDeviceError> {
    let RegisterDeviceInput {
        refresh_token,
        user_id,
        user_agent,
    } = input;

    let device = UserDevice {
        uid: util::id_generator::generate_id("device"),
        hashed_token: crypto::hash_token(&refresh_token),
        user_id,
        user_agent,
        last_accessed_at: Utc::now(),
    };

    match create_device(&deps.db, device).await {
        Ok(_) => Ok(()),
        Err(SaveDeviceError::DeviceAlreadyExists) => Err(RegisterDeviceError::DeviceAlreadyExists),
        Err(SaveDeviceError::InternalError(message)) => {
            Err(RegisterDeviceError::InternalError(message))
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum InvalidateDeviceByTokenError {
    #[error("Device not found")]
    DeviceNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn invalidate_device_by_token(
    deps: &AppDeps,
    refresh_token: &str,
) -> Result<(), InvalidateDeviceByTokenError> {
    let hashed_token = crypto::hash_token(refresh_token);

    match invalidate_device(&deps.db, &hashed_token).await {
        Ok(()) => Ok(()),
        Err(InvalidateDeviceError::DeviceNotFound) => {
            Err(InvalidateDeviceByTokenError::DeviceNotFound)
        }
        Err(InvalidateDeviceError::InternalError(message)) => {
            Err(InvalidateDeviceByTokenError::InternalError(message))
        }
    }
}
