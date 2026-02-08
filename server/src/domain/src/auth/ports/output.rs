use crate::prelude::{DomainResult, Secret};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordVerifierPortError {
    InternalError(String),
    PasswordNotMatch,
}

pub trait PasswordVerifierPort {
    fn verify(
        &self,
        proposed_password: &Secret,
        confirmed_password: &Secret,
    ) -> impl Future<Output = DomainResult<(), PasswordVerifierPortError>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHasherPortError {
    InternalError(String),
}

pub trait PasswordHasherPort {
    fn hash(
        &self,
        password: &Secret,
    ) -> impl Future<Output = DomainResult<Secret, PasswordHasherPortError>>;
}
