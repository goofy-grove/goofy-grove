use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum CreateDeviceError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device already exists")]
    DeviceAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[derive(Debug, Clone, Error)]
pub enum InvalidateDeviceError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device not found")]
    DeviceNotFound,
}

pub trait CreateDeviceUseCase {
    fn create_device(
        &self,
        command: CreateDeviceCommand,
    ) -> impl Future<Output = Result<UserToken, CreateDeviceError>>;
}

pub trait InvalidateDeviceUseCase {
    fn invalidate_device(
        &self,
        command: InvalidateDeviceCommand,
    ) -> impl Future<Output = Result<(), InvalidateDeviceError>>;
}
