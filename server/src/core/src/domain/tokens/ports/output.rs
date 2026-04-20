use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum LoadDevicePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device not found")]
    DeviceNotFound,
}

pub trait LoadDevicePort {
    fn load_device(
        &self,
        hashed_token: &HashedToken,
    ) -> impl Future<Output = Result<UserToken, LoadDevicePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum SaveDevicePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device already exists")]
    DeviceAlreadyExists,

    #[error("User not found")]
    UserNotFound,
}

pub trait SaveDevicePort {
    fn create_device(
        &self,
        user_token: UserToken,
    ) -> impl Future<Output = Result<UserToken, SaveDevicePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum InvalidateDevicePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Device not found")]
    DeviceNotFound,
}

pub trait InvalidateDevicePort {
    fn invalidate_device(
        &self,
        hashed_token: &HashedToken,
    ) -> impl Future<Output = Result<(), InvalidateDevicePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum TokenHasherPortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait TokenHasherPort {
    fn hash_token(
        &self,
        token: Token,
    ) -> impl Future<Output = Result<HashedToken, TokenHasherPortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TokenGeneratorPortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait TokenGeneratorPort {
    fn generate_token(
        &self,
        user: &User,
    ) -> impl Future<Output = Result<(String, usize), TokenGeneratorPortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TokenValidatorPortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Invalid token")]
    TokenInvalid,

    #[error("Token expired")]
    TokenExpired,
}

pub trait TokenValidatorPort {
    fn validate_token(
        &self,
        token: &Token,
    ) -> impl Future<Output = Result<TokenData, TokenValidatorPortError>>;
}
