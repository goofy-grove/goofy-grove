use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum LoadDevicePortError {
    InternalError(String),
    DeviceNotFound,
}

pub trait LoadDevicePort {
    fn load_device(
        &self,
        hashed_token: &HashedToken,
    ) -> impl Future<Output = DomainResult<UserToken, LoadDevicePortError>>;
}

#[derive(Debug, Clone)]
pub enum SaveDevicePortError {
    InternalError(String),
    DeviceAlreadyExists,
    UserNotFound,
}

pub trait SaveDevicePort {
    fn create_device(
        &self,
        user_token: UserToken,
    ) -> impl Future<Output = DomainResult<UserToken, SaveDevicePortError>>;
}

#[derive(Debug, Clone)]
pub enum InvalidateDevicePortError {
    InternalError(String),
    DeviceNotFound,
}

pub trait InvalidateDevicePort {
    fn invalidate_device(
        &self,
        hashed_token: &HashedToken,
    ) -> impl Future<Output = DomainResult<(), InvalidateDevicePortError>>;
}

#[derive(Debug, Clone)]
pub enum TokenHasherPortError {
    InternalError(String),
}

pub trait TokenHasherPort {
    fn hash_token(&self, token: Token) -> impl Future<Output = DomainResult<HashedToken, TokenHasherPortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenGeneratorPortError {
    InternalError(String),
}

pub trait TokenGeneratorPort {
    fn generate_token(
        &self,
        user: &User,
    ) -> impl Future<Output = DomainResult<(String, usize), TokenGeneratorPortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenValidatorPortError {
    InternalError(String),
    TokenInvalid,
    TokenExpired,
}

pub trait TokenValidatorPort {
    fn validate_token(
        &self,
        token: &Token,
    ) -> impl Future<Output = DomainResult<TokenData, TokenValidatorPortError>>;
}
