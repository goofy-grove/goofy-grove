use crate::domain::{
    error::DomainResult,
    prelude::{CreateDeviceCommand, InvalidateDeviceCommand, UserToken},
};

#[derive(Debug, Clone)]
pub enum CreateDeviceError {
    InternalError(String),
    DeviceAlreadyExists,
    UserNotFound,
}

#[derive(Debug, Clone)]
pub enum InvalidateDeviceError {
    InternalError(String),
    DeviceNotFound,
}

pub trait CreateDeviceUseCase {
    fn create_device(
        &self,
        command: CreateDeviceCommand,
    ) -> impl Future<Output = DomainResult<UserToken, CreateDeviceError>>;
}

pub trait InvalidateDeviceUseCase {
    fn invalidate_device(
        &self,
        command: InvalidateDeviceCommand,
    ) -> impl Future<Output = DomainResult<(), InvalidateDeviceError>>;
}
