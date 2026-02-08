use crate::{error::DomainResult, prelude::Secret};

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
