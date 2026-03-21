use crate::domain::{
    error::DomainResult,
    prelude::{Token, UserToken},
};

#[derive(Debug, Clone)]
pub enum GetDeviceError {
    InternalError(String),
    DeviceNotFound,
}

pub trait GetDeviceQuery {
    fn get_device(
        &self,
        token: &Token,
    ) -> impl Future<Output = DomainResult<UserToken, GetDeviceError>>;
}
