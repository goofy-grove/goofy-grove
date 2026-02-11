use crate::domain::prelude::*;

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
