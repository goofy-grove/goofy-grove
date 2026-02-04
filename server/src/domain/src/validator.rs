use crate::error::DomainError;

pub type DomainValidationResult = Result<(), DomainError>;

pub trait Validator {
    fn validate(&self) -> DomainValidationResult;
}
